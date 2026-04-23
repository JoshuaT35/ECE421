# ECE421 Poker Project

This folder contains the ECE 421 poker project workspaces:

- `poker-project`
- `poker-client-project`

Both folders contain the same poker workspace structure. If you just want to run the project, use `poker-project`.

## Project Layout

From `ECE421/PokerProject/poker-project`, the workspace includes:

- `poker-server`
- `poker-client`
- `poker-model`
- `database-handler`

## How To Run

1. Open a terminal in:

```bash
cd ECE421/PokerProject/poker-project
```

2. Build the workspace:

```bash
cargo build
```

3. Start the server:

```bash
cargo run -p poker-server
```

The server listens on `0.0.0.0:6000`.

4. Find the server machine's IP address.

On Linux or macOS:

```bash
ifconfig
```

On Windows:

```bash
ipconfig
```

5. Start a client in a new terminal:

```bash
cd ECE421/PokerProject/poker-project
cargo run -p poker-client -- <server-ip>:6000
```

Example:

```bash
cargo run -p poker-client -- 192.168.1.25:6000
```

6. Start at least one more client the same way so multiple players can join.

Each client should run in its own terminal window. You can run the server and clients on one machine with multiple terminals, or across multiple machines on the same network.

## Notes

- The server starts the game flow automatically after clients connect.
- If clients cannot connect, check that port `6000` is not blocked by the host firewall.
- `poker-client` is the correct package name for the GUI client.
