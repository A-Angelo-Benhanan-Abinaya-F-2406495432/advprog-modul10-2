### 2.1 Original Code

Server:
![Server](assets/2.1-server.png)
Client:
1. ![Client 1](assets/2.1-client1.png)
2. ![Client 1](assets/2.1-client2.png)
3. ![Client 1](assets/2.1-client3.png)

How to run:
- Server: cargo build --bin server
- Client: cargo build --bin client

When each client connects, the server logs the new connection with its assigned port (62439, 62460, 62467).
When a client types a message and presses Enter, the message is sent to the server over a WebSocket connection. The server logs it and broadcasts it to all connected clients.
Each client therefore sees every message sent by any participant, regardless of who sent it.
