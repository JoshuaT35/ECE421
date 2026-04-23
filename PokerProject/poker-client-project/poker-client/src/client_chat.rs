/// chat portion of the GUI

use eframe::{
    egui::{self, Button, CentralPanel, Label, ScrollArea, TextEdit},
    App,
};
use crate::client_communication::TcpClient;
use std::sync::{Arc, Mutex};

pub struct ClientChat {
    tcp_client: TcpClient,
    input_message: String,
}

impl ClientChat {
    pub fn new(tcp_client: TcpClient) -> Self {
        Self {
            tcp_client,
            input_message: String::new(),
        }
    }
}

impl App for ClientChat {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("TCP Chat Client");

            // Display messages
            ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                let messages = self.tcp_client.get_messages();
                let messages_lock = messages.lock().unwrap();
                for msg in messages_lock.iter() {
                    ui.add(Label::new(msg.clone()));
                }
            });

            // Input field
            ui.add(TextEdit::singleline(&mut self.input_message).hint_text("Type a message..."));

            // Send button
            if ui.add(Button::new("Send")).clicked() {
                self.tcp_client.send_message(self.input_message.clone());
                self.input_message.clear();
            }

            if ui.add(Button::new("Back to Menu")).clicked() {
                *self = ClientMenu::new(self.tcp_client.clone()).into();
            }
        });

        ctx.request_repaint();
    }
}
