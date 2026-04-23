/// menu portion of the GUI

use eframe::egui::{self, Button, CentralPanel};
use eframe::App;
use crate::client_communication::TcpClient;
use crate::client_chat::ClientChat;

pub struct ClientMenu {
    tcp_client: TcpClient,
}

impl ClientMenu {
    pub fn new(tcp_client: TcpClient) -> Self {
        Self { tcp_client }
    }
}

impl App for ClientMenu {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Main Menu");

            if ui.add(Button::new("Enter Chat")).clicked() {
                *self = ClientChat::new(self.tcp_client.clone()).into();
            }
        });

        ctx.request_repaint();
    }
}
