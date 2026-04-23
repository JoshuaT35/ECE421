# ECE 421 Project 1 + 3

## Summary
This is a Rust-written project that intends to simulate three distinct variants of poker.
The poker game supports multiple devices connecting and communicating with the server as long as all client devices are on the same wifi as the server and know the server device's IP. This allows for multiple players to play a game of poker simultaneously. Multiple players can also be connected from the device hosting the server if desired.

## How to Run
As with poker, there is a dealer, and at least 2 players depending on the poker variant.
To run this project, a user must run the server and at least 2 client connections.

### Steps
1. Gather at least 3 devices, one to run the server, and two to participate as players. From here one, these devies will be referred to as Server, Client 1, and Client 2 respectively. These devices must be connected to the same wifi network.
2. On all devices, run `cargo build`.
3. On Server's terminal, run `cargo run -p poker-server` to begin the server with the dealer.
4. The server is automated, and does not need manual interaction anymore.
5. In a new terminal but on the same machine hosting the server, run the command `ipconfig` in Windows or `ifconfig` in Mac/Linux. NOTE: Firewall settings may need to be adjusted to open port 6000 on the server device or allow this program as an app that can bypass the firewall.
5. On Client 1 and Client 2, run `cargo run -p player-client -- x.x.x.x:6000` to begin a client where `x.x.x.x` is the IPv4 address of the device hosting the server retrieved from above.  This client automatically connects to the server.
6. To add additional devies, repeat step 5 as many times as you like to simulate a new player each time. Note that each client must have its own terminal to host a unique connection with the server.
7. Follow the prompts on each client, and enjoy a game of poker!

### NOTES
- This project can work with multiple terminals on the same device, or with multiple devices
- Unique networking protocols may have to be setup for different configurations, especially if they are not on the same LAN or WAN.
