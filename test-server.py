#!/usr/bin/env python3
"""Quick test server to verify counter works"""
from http.server import HTTPServer, SimpleHTTPRequestHandler
import urllib.parse

class TestHandler(SimpleHTTPRequestHandler):
    counter = 0

    def do_POST(self):
        path = urllib.parse.urlparse(self.path).path

        if path == '/increment':
            TestHandler.counter += 1
            self.send_response(200)
            self.send_header('Content-type', 'text/html')
            self.send_header('Access-Control-Allow-Origin', '*')
            self.end_headers()
            self.wfile.write(str(TestHandler.counter).encode())
            print(f"✅ Increment: counter = {TestHandler.counter}")

        elif path == '/decrement':
            TestHandler.counter -= 1
            self.send_response(200)
            self.send_header('Content-type', 'text/html')
            self.send_header('Access-Control-Allow-Origin', '*')
            self.end_headers()
            self.wfile.write(str(TestHandler.counter).encode())
            print(f"✅ Decrement: counter = {TestHandler.counter}")

        elif path == '/reset':
            TestHandler.counter = 0
            self.send_response(200)
            self.send_header('Content-type', 'text/html')
            self.send_header('Access-Control-Allow-Origin', '*')
            self.end_headers()
            self.wfile.write(str(TestHandler.counter).encode())
            print(f"✅ Reset: counter = {TestHandler.counter}")
        else:
            super().do_POST()

    def end_headers(self):
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type, RX-Request')
        super().end_headers()

if __name__ == '__main__':
    import os
    os.chdir('examples/basic')
    print("🚀 Test server running at http://localhost:8000")
    print("📝 Test at: http://localhost:8000/test.html")
    print("⚡ Counter at: http://localhost:8000/counter.html")
    HTTPServer(('', 8000), TestHandler).serve_forever()
