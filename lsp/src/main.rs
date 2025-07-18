use std::error::Error;
use d_compiler::lexer::lexer::Lexer;
use d_compiler::lexer::token::{Token, TokenInfo};

use lsp_server::{Connection, Message, Request, RequestId, Response};
use lsp_types::{
    DidChangeTextDocumentParams,
    InitializeParams, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind,
};
use lsp_types::notification::{PublishDiagnostics, Notification};

fn main() -> Result<(), Box<dyn Error>> {
    // Create the transport
    let (connection, io_threads) = Connection::stdio();

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit)
    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(
            TextDocumentSyncKind::FULL,
        )),
        completion_provider: Some(lsp_types::CompletionOptions {
            resolve_provider: Some(false),
            trigger_characters: Some(vec![".".to_string()]),
            ..Default::default()
        }),
        ..ServerCapabilities::default()
    })
    .unwrap();
    let initialization_params = connection.initialize(server_capabilities)?;
    let _params: InitializeParams = serde_json::from_value(initialization_params).unwrap();

    while let Ok(msg) = connection.receiver.recv() {
        eprintln!("got msg: {:?}", msg);
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    break;
                }
                match req.method.as_str() {
                    "textDocument/hover" => {
                        let (id, params) = cast::<lsp_types::request::HoverRequest>(req)?;
                        let uri = params.text_document_position_params.text_document.uri;
                        let file_path = uri.to_file_path().unwrap();
                        let content = std::fs::read_to_string(file_path).unwrap();
                        let position = params.text_document_position_params.position;
                        let mut lexer = Lexer::new(&content);
                        let tokens = lexer.tokenize().unwrap_or_default();

                        let token = tokens.iter().find(|token_info| {
                            let token_line = token_info.line as u32 - 1;
                            let token_start_column = token_info.column as u32 - 1;
                            let token_end_column = token_start_column + token_info.lexeme.len() as u32;

                            position.line == token_line
                                && position.character >= token_start_column
                                && position.character <= token_end_column
                        });

                        let hover_content = token.map_or_else(
                            || "No token found".to_string(),
                            |token_info| format_token_info(token_info),
                        );

                        let result = Some(lsp_types::Hover {
                            contents: lsp_types::HoverContents::Scalar(
                                lsp_types::MarkedString::String(hover_content),
                            ),
                            range: None,
                        });

                        let result = serde_json::to_value(&result).unwrap();
                        let resp = Response { id, result: Some(result), error: None };
                        if let Some(result) = resp.result {
                            connection.sender.send(Message::Response(Response {
                                id: resp.id,
                                result: Some(result),
                                error: None,
                            }))?;
                        }
                    }
                    "textDocument/completion" => {
                        let (id, _params) = cast::<lsp_types::request::Completion>(req)?;
                        let completions = get_completions();
                        let result = Some(lsp_types::CompletionResponse::Array(completions));
                        let result = serde_json::to_value(&result).unwrap();
                        let resp = Response { id, result: Some(result), error: None };
                        connection.sender.send(Message::Response(resp))?;
                    }
                    _ => (),
                }
            }
            Message::Response(resp) => {
                if let Some(err) = resp.error {
                    eprintln!("got error: {:?}", err);
                }
            }
            Message::Notification(not) => {
                match not.method.as_str() {
                    "textDocument/didChange" => {
                        let params: DidChangeTextDocumentParams = serde_json::from_value(not.params).unwrap();
                        let uri = params.text_document.uri;
                        let content = params.content_changes[0].text.clone();
                        let diagnostics = get_diagnostics(&content);
                        let params = lsp_types::PublishDiagnosticsParams {
                            uri,
                            diagnostics,
                            version: None,
                        };
                        let notification = lsp_server::Notification::new(PublishDiagnostics::METHOD.to_string(), params);
                        connection.sender.send(Message::Notification(notification))?;
                    }
                    _ => (),
                }
            }
        }
    }

    io_threads.join()?;
    Ok(())
}

fn cast<R>(req: Request) -> Result<(RequestId, R::Params), Box<dyn Error>>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    let params = serde_json::from_value(req.params).unwrap();
    Ok((req.id, params))
}

fn format_token_info(token_info: &TokenInfo) -> String {
    let token_type = match &token_info.token {
        Token::Reserved(r) => format!("Reserved keyword: `{:?}`", r),
        Token::Identifier(id) => format!("Identifier: `{}`", id),
        Token::String(s) => format!("String literal: `\"{}\"`", s),
        Token::Number(n) => format!("Number literal: `{}`", n),
        Token::Operation(op) => format!("Operator: `{:?}`", op),
        Token::Punctuation(p) => format!("Punctuation: `{:?}`", p),
        Token::Whitespace => "Whitespace".to_string(),
        Token::Newline => "Newline".to_string(),
        Token::Invalid(i) => format!("Invalid token: `{}`", i),
        Token::Eof => "End of file".to_string(),
    };
    format!("```\n{}\n```", token_type)
}

fn get_diagnostics(content: &str) -> Vec<lsp_types::Diagnostic> {
    match d_compiler::compile(content) {
        Ok(_) => Vec::new(),
        Err(err) => {
            let range = lsp_types::Range {
                start: lsp_types::Position {
                    line: 0,
                    character: 0,
                },
                end: lsp_types::Position {
                    line: 0,
                    character: 0,
                },
            };
            let diagnostic = lsp_types::Diagnostic {
                range,
                severity: Some(lsp_types::DiagnosticSeverity::ERROR),
                message: err,
                ..Default::default()
            };
            vec![diagnostic]
        }
    }
}

fn get_completions() -> Vec<lsp_types::CompletionItem> {
    vec![
        lsp_types::CompletionItem {
            label: "true".to_string(),
            kind: Some(lsp_types::CompletionItemKind::KEYWORD),
            ..Default::default()
        },
        lsp_types::CompletionItem {
            label: "false".to_string(),
            kind: Some(lsp_types::CompletionItemKind::KEYWORD),
            ..Default::default()
        },
        lsp_types::CompletionItem {
            label: "if".to_string(),
            kind: Some(lsp_types::CompletionItemKind::KEYWORD),
            ..Default::default()
        },
        lsp_types::CompletionItem {
            label: "else".to_string(),
            kind: Some(lsp_types::CompletionItemKind::KEYWORD),
            ..Default::default()
        },
    ]
}
