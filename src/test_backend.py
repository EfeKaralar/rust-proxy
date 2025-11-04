#!/usr/bin/env python3
from http.server import HTTPServer, BaseHTTPRequestHandler

class SimpleHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200)
        self.send_header('Content-type', 'text/plain')
        self.end_headers()
        self.wfile.write(b'Hello from backend server!\n')
    
    def log_message(self, format, *args):
        print(f"Backend received: {format % args}")

if __name__ == '__main__':
    server = HTTPServer(('127.0.0.1', 3000), SimpleHandler)
    print("Backend server running on port 3000")
    server.serve_forever()
