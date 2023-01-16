# echo-client.py

import socket

HOST = "127.0.0.1"  # The server's hostname or IP address
PORT = 80  # The port used by the server

print("Going to connect on network")

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect((HOST, PORT))
    s.sendall(b"Hello, world ")

    while True:
        data = s.recv(1024)
        print(f"Received {data!r}")
