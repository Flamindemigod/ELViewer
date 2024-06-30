mod modules;
use anyhow::Result;
use gtk::prelude::*;
use gtk::{glib, Application};
use modules::backend::Backend;
use modules::channel::{interaction_channel, ClientInteractions, Clients, ServerInteractions};
use modules::frontend::spawn_frontend;
use std::{fs::File, io::BufRead};

use modules::ui::build_ui;

const APP_ID: &str = "org.flamindemigod.ElViewer";

#[tokio::main]
async fn main() -> Result<()> {
    let mut backend = Backend::init();
    let (mut server_side, client_side_generator) = interaction_channel(1);

    let _fe = tokio::spawn(spawn_frontend(
        APP_ID,
        client_side_generator(&mut server_side, Clients::Frontend),
    ));

    while let Some(req) = server_side.recieve.recv().await {
        match req {
            ClientInteractions::ParseLog(path) => {
                &backend.parse_file(path);
                server_side.respond(Clients::Frontend, ServerInteractions::ParseLog)
            }
            ClientInteractions::LogProgress => server_side.respond(
                Clients::Frontend,
                ServerInteractions::LogProgress(backend.poll_progress().unwrap()),
            ), // ClientInteractions::WsSocket => server_side.respond(
               //     Clients::WebSocket,
               //     ServerInteractions::WsSocket(server.get_addr_websocket()),
               // ),
               // ClientInteractions::WsValidateClient(token) => server_side.respond(
               //     Clients::WebSocket,
               //     ServerInteractions::WsValidateClient(server.is_client_valid(token.as_str())),
               // ),
               // ClientInteractions::WsClientConnected { addr, client } => {
               //     server.client_connected(addr, client);
               //     server_side.respond(Clients::WebSocket, ServerInteractions::WsClientConnected)
               // }
               // ClientInteractions::WsGetConnectedClients => server_side.respond(
               //     Clients::WebSocket,
               //     ServerInteractions::WsGetConnectedClients(server.get_connected_clients()),
               // ),
               // ClientInteractions::WsSetClientConnectedTx { addr, tx } => {
               //     server.set_connected_client_tx(&addr, tx);
               //     server_side.respond(
               //         Clients::WebSocket,
               //         ServerInteractions::WsSetClientConnectedTx,
               //     );
               // }
               // ClientInteractions::WsClientLeft { addr } => {
               //     server.client_disconnected(&addr);
               //     server_side.respond(Clients::WebSocket, ServerInteractions::WsClientLeft);
               // }
               //
               // ClientInteractions::HttpSocket => server_side.respond(
               //     Clients::Http,
               //     ServerInteractions::HttpSocket(server.get_addr_http()),
               // ),
               //
               // ClientInteractions::HttpValidateClient(token) => server_side.respond(
               //     Clients::Http,
               //     ServerInteractions::HttpValidateClient(
               //         server.is_client_connected_by_token(token.as_str()),
               //     ),
               // ),
               //
               // ClientInteractions::HttpGetAllClients => server_side.respond(
               //     Clients::Http,
               //     ServerInteractions::HttpGetAllClients(server.get_all_clients()),
               // ),
               // ClientInteractions::HttpGetConnectedClients => server_side.respond(
               //     Clients::Http,
               //     ServerInteractions::HttpGetConnectedClients(server.get_connected_clients()),
               // ),
        };
    }
    Ok(())
}
