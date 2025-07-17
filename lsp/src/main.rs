use std::error::Error;
use d_compiler::lexer::lexer::Lexer;
use d_compiler::lexer::token::{Token, TokenInfo};

use lsp_server::{Connection, Message, Request, RequestId, Response};
use lsp_types::{
    InitializeParams, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Create the transport
    let (connection, io_threads) = Connection::stdio();

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit)
    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(
            TextDocumentSyncKind::FULL,
        )),
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
                        let tokens = lexer.tokenize();

                        let token = tokens.iter().find(|token| {
                            let token_line = token.line as u32;
                            let token_start_column = token.column as u32;
                            let token_end_column = token_start_column + token.lexeme.len() as u32;

                            position.line == token_line
                                && position.character >= token_start_column
                                && position.character <= token_end_column
                        });

                        let hover_content = token.map_or_else(
                            || "No token found".to_string(),
                            |token| format_token_info(token),
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
                    _ => (),
                }
            }
            Message::Response(resp) => {
                if let Some(err) = resp.error {
                    eprintln!("got error: {:?}", err);
                }
            }
            Message::Notification(not) => {
                eprintln!("got notification: {:?}", not);
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
