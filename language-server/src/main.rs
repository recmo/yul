#![doc = include_str!("../Readme.md")]
#![warn(clippy::all, clippy::pedantic, clippy::cargo, clippy::nursery)]

use anyhow::Result;
use flexi_logger::{LogTarget, Logger};
use log::{debug, info};
use lsp_server::{Connection, Message, Request, RequestId, Response};
use lsp_types::{
    request::GotoDefinition, GotoDefinitionResponse, InitializeParams, ServerCapabilities,
};

#[derive(Clone, Debug, StructOpt)]
struct Options {}

fn main_loop(connection: &Connection, params: serde_json::Value) -> Result<()> {
    let _params: InitializeParams = serde_json::from_value(params).unwrap();
    debug!("starting example main loop");
    for msg in &connection.receiver {
        debug!("got msg: {:?}", msg);
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                debug!("got request: {:?}", req);
                match cast::<GotoDefinition>(req) {
                    Ok((id, params)) => {
                        debug!("got gotoDefinition request #{}: {:?}", id, params);
                        let result = Some(GotoDefinitionResponse::Array(Vec::new()));
                        let result = serde_json::to_value(&result).unwrap();
                        let resp = Response {
                            id,
                            result: Some(result),
                            error: None,
                        };
                        connection.sender.send(Message::Response(resp))?;
                        continue;
                    }
                    Err(req) => req,
                };
                // ...
            }
            Message::Response(resp) => {
                debug!("got response: {:?}", resp);
            }
            Message::Notification(not) => {
                debug!("got notification: {:?}", not);
            }
        }
    }
    Ok(())
}

fn cast<R>(req: Request) -> Result<(RequestId, R::Params), Request>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    req.extract(R::METHOD)
}

async fn app(options: Options) -> Result<()> {
    // Note that  we must have our logging only write out to stderr.
    Logger::with_env()
        .log_target(LogTarget::StdErr)
        .start()
        .unwrap();

    info!("starting Yul LSP server");

    // Create the transport. Includes the stdio (stdin and stdout) versions but this
    // could also be implemented to use sockets or HTTP.
    let (connection, io_threads) = Connection::stdio();

    // Run the server and wait for the two threads to end (typically by trigger LSP
    // Exit event).
    let server_capabilities = serde_json::to_value(&ServerCapabilities::default()).unwrap();
    let initialization_params = connection.initialize(server_capabilities)?;
    main_loop(&connection, initialization_params)?;
    io_threads.join()?;

    // Shut down gracefully.
    info!("shutting down server");
    Ok(())
}

fn main() {
    cli_batteries::run(version!(), app);
}
