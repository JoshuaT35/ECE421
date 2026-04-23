/// registration portion of the GUI

use eframe::egui::{self, Button, CentralPanel, TextEdit, Label, ScrollArea};
use eframe::App;

pub struct RegistrationClient {
    username: String,
    received_prompt: bool,
    login_success: bool,
}

impl RegistrationClient {
    pub fn new() -> Self {
        Self {
            username: String::new(),
            received_prompt: false,
            login_success: false,
        }
    }

    /// Public method to check login status
    pub fn is_logged_in(&self) -> bool {
        self.login_success
    }

    /// Handles login logic using messages from `ClientApp`
    pub fn update_login(&mut self, messages: &Vec<String>) {
        for msg in messages.iter() {
            if msg.contains("Please enter your name") {
                self.received_prompt = true;
            }
            if msg.contains("Welcome") {
                self.login_success = true;
            }
        }
    }
}

impl App for RegistrationClient {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Login to Game");

            if self.received_prompt {
                ui.label("Username:");
                ui.add(TextEdit::singleline(&mut self.username));

                if ui.add(Button::new("Login")).clicked() {
                    println!("Sending username: {}", self.username);
                }
            } else {
                ui.label("Waiting for server...");
            }
        });

        ctx.request_repaint();
    }
}
