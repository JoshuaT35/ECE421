mod server_utils;
mod server_communication;
mod game_logic;
mod server_registration;
mod server_lobby;
mod server_games;
mod game_server;

fn main() {
    let server = game_server::Server::new("0.0.0.0:6000");
    server.run();
}
