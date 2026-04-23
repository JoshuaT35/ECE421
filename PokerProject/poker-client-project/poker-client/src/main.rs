mod client_communication;
mod client_app;

use eframe::NativeOptions;
use crate::client_app::ClientApp;

//   Windows: Run ipconfig on device that's hosting server
//   Mac/Linux: ifconfig on device that's hosting server
// Take the IPv4 address and then pass the IP as a command line argument:
// cargo run -p poker-client -- serverIP:6000  
// Where serverIP would be something like 192.168.x.x

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let server_ip = args.get(1).unwrap();
    // let server_ip = "0.0.0.0:6000";

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Poker Client",
        native_options,
        Box::new(|_cc| Ok(Box::new(ClientApp::new(server_ip)))),
    );
}