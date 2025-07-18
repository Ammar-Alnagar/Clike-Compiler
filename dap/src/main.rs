use std::error::Error;
use std::io::{stdin, stdout, Read, Write};
use dap::prelude::{Adapter, Context, Request, Response, Server, Command, ResponseBody};
use dap::types::Capabilities;

struct MyAdapter;

impl Adapter for MyAdapter {
    fn accept(&mut self, request: &Request, _ctx: &mut dyn Context) -> anyhow::Result<Response> {
        eprintln!("Accepting request: {:?}", request);
        match &request.command {
            Command::Initialize(_) => {
                let capabilities = Capabilities {
                    supports_configuration_done_request: Some(true),
                    ..Default::default()
                };
                Ok(Response::make_success(request,ResponseBody::Initialize(capabilities)))
            }
            _ => todo!(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let adapter = Box::new(MyAdapter);
    let mut server = Server::new(adapter, stdout());
    let mut reader = stdin();
    let mut buffer = Vec::new();
    loop {
        let mut buf = [0; 1024];
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        buffer.extend_from_slice(&buf[..n]);
        let mut new_buffer = Vec::new();
        match server.process(&buffer) {
            Ok(Some(remaining)) => {
                new_buffer.extend_from_slice(remaining);
            }
            Ok(None) => {}
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
        buffer = new_buffer;
    }
    Ok(())
}
