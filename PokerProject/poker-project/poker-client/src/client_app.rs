/// Client GUI

use eframe::{
    egui::{self, Button, CentralPanel, Label, ScrollArea, TextEdit, Color32, RichText},
    App, Frame,
};
use std::sync::{Arc, Mutex};
use crate::client_communication::TcpClient;

// App states
pub enum AppState {
    Login,
    Menu,
    Chat,
}

pub struct ClientApp {
    tcp_client: TcpClient,
    input_message: String,
    state: AppState,
}

impl ClientApp {
    pub fn new(address: &str) -> Self {
        let tcp_client = TcpClient::new(address);
        Self {
            tcp_client,
            input_message: String::new(),
            state: AppState::Login, // Start in Login mode
        }
    }
}

impl App for ClientApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        match &self.state {
            AppState::Login => self.show_login(ctx),
            AppState::Menu => self.show_menu(ctx),
            AppState::Chat => self.show_chat(ctx),
        }

        ctx.request_repaint();
    }
}

impl ClientApp {
    /// Handles the login screen (server asks for name in chat format)
    fn show_login(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Login to TCP Chat");

            let messages = self.tcp_client.get_messages();
            let messages_lock = messages.lock().unwrap();

            let mut received_prompt = false;
            let mut login_successful = false;

            // Display chat messages from the server
            ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                for msg in messages_lock.iter() {
                    ui.add(Label::new(msg.clone()));

                    // Check if the server is prompting for a name
                    if msg.contains("Please enter your name") {
                        received_prompt = true;
                    }

                    // Check if login was successful
                    if msg.contains("Welcome") {
                        login_successful = true;
                    }
                }
            });

            // If the server has asked for a name, allow input
            if received_prompt {
                ui.add(TextEdit::singleline(&mut self.input_message).hint_text("Type your name..."));

                if ui.add(Button::new("Login")).clicked() {
                    self.tcp_client.send_message(self.input_message.clone());
                    self.input_message.clear();
                }
            } else {
                ui.label("Waiting for server...");
            }

            // Move to Menu **only** when login is confirmed
            if login_successful {
                self.state = AppState::Menu;
            }
        });

        ctx.request_repaint();
    }

    /// Handles the lobby after login
    fn show_menu(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Lobby");

            ui.vertical_centered(|ui| {
                ui.heading(RichText::new("Lobby").size(28.0).color(Color32::BLACK));
                ui.add_space(10.0);

                if ui.button("Play Local Game").clicked() {
                    self.state = AppState::Chat;
                }

                if ui.button("Logout").clicked() {
                    self.state = AppState::Login;
                }
            });
        });
    }

    /// Handles the actual chat interface
    fn show_chat(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("TCP Chat Client");

            ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                let messages = self.tcp_client.get_messages();
                let messages_lock = messages.lock().unwrap();
                for msg in messages_lock.iter() {
                    ui.add(Label::new(msg.clone()));
                }
            });

            ui.add(TextEdit::singleline(&mut self.input_message).hint_text("Type a message..."));

            if ui.add(Button::new("Send")).clicked() {
                self.tcp_client.send_message(self.input_message.clone());
                self.input_message.clear();
            }

            if ui.add(Button::new("Back to Lobby")).clicked() {
                self.state = AppState::Menu;
            }
        });
    }
}




                // if ui.button("Spectate Game").clicked() {
                //     self.is_spectator = true;
                //     self.state = State::Spectator;
                //     self.initialize_spectator_mode();
                // }
                // if ui.button("View Stats").clicked() {
                //     self.state = State::Stats;
                // }
                // if ui.button("Help / Rules").clicked() {
                //     self.state = State::Help;
                //     self.help_variant = None;
                // }

            // if ui.add(Button::new("Enter Chat")).clicked() {
            //     self.state = AppState::Chat;
            // }

            // if ui.add(Button::new("Logout")).clicked() {
            //     self.state = AppState::Login;
            // }