#!/usr/bin/env python3
"""
Simple HTTP server for rustx-wasm examples.
Handles API endpoints and serves static files.
"""

from http.server import HTTPServer, SimpleHTTPRequestHandler
import json
import urllib.parse
from datetime import datetime
import sys

class RustXHandler(SimpleHTTPRequestHandler):
    """Custom handler for rustx-wasm API endpoints"""

    # Counter state (shared across all requests)
    counter = 0

    def do_GET(self):
        """Handle GET requests"""
        parsed_path = urllib.parse.urlparse(self.path)
        path = parsed_path.path

        # API endpoints
        if path == '/api/hello':
            self.send_html_response(f'<strong>Hello from RustX!</strong> Request received at {datetime.now().strftime("%H:%M:%S")}')

        elif path == '/api/item':
            item_num = datetime.now().microsecond % 100
            self.send_html_response(f'<div>New Item #{item_num}</div>')

        elif path == '/api/clear':
            self.send_html_response('<div>List cleared!</div>')

        elif path.startswith('/api/select'):
            query = urllib.parse.parse_qs(parsed_path.query)
            value = query.get('value', ['none'])[0]
            if value:
                self.send_html_response(f'<strong>You selected:</strong> {value}')
            else:
                self.send_html_response('Please select an option...')

        else:
            # Serve static files
            super().do_GET()

    def do_POST(self):
        """Handle POST requests"""
        parsed_path = urllib.parse.urlparse(self.path)
        path = parsed_path.path

        # Read request body
        content_length = int(self.headers.get('Content-Length', 0))
        body = self.rfile.read(content_length)

        if path == '/api/submit':
            # Parse form data
            try:
                form_data = urllib.parse.parse_qs(body.decode('utf-8'))
                username = form_data.get('username', [''])[0]
                email = form_data.get('email', [''])[0]

                response_html = f'''
                <div style="color: #28a745;">
                    <strong>Form submitted successfully!</strong><br>
                    Username: {username}<br>
                    Email: {email}<br>
                    Time: {datetime.now().strftime("%Y-%m-%d %H:%M:%S")}
                </div>
                '''
                self.send_html_response(response_html)
            except Exception as e:
                self.send_html_response(f'<div style="color: red;">Error: {str(e)}</div>')

        elif path == '/api/create':
            form_data = urllib.parse.parse_qs(body.decode('utf-8'))
            action = form_data.get('action', ['unknown'])[0]
            item_id = datetime.now().microsecond

            response_html = f'''
            <div style="color: #007bff;">
                <strong>Item created!</strong><br>
                Action: {action}<br>
                ID: {item_id}<br>
                Timestamp: {datetime.now().strftime("%H:%M:%S")}
            </div>
            '''
            self.send_html_response(response_html)

        # Counter endpoints
        elif path == '/increment':
            RustXHandler.counter += 1
            self.send_html_response(str(RustXHandler.counter))

        elif path == '/decrement':
            RustXHandler.counter -= 1
            self.send_html_response(str(RustXHandler.counter))

        elif path == '/reset':
            RustXHandler.counter = 0
            self.send_html_response(str(RustXHandler.counter))

        else:
            self.send_error(404, "Endpoint not found")

    def send_html_response(self, html_content):
        """Send an HTML response"""
        self.send_response(200)
        self.send_header('Content-type', 'text/html; charset=utf-8')
        self.send_header('Access-Control-Allow-Origin', '*')
        self.end_headers()
        self.wfile.write(html_content.encode('utf-8'))

    def end_headers(self):
        """Add CORS headers"""
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type, RX-Request')
        super().end_headers()

    def log_message(self, format, *args):
        """Custom log format"""
        sys.stderr.write(f"[{datetime.now().strftime('%Y-%m-%d %H:%M:%S')}] {format % args}\n")


def run_server(port=8000):
    """Start the HTTP server"""
    server_address = ('', port)
    httpd = HTTPServer(server_address, RustXHandler)

    print(f"""
    ╔════════════════════════════════════════════╗
    ║     RustX WASM Example Server Running     ║
    ╚════════════════════════════════════════════╝

    🌐 Server: http://localhost:{port}
    📝 Examples:
       • http://localhost:{port}/index.html
       • http://localhost:{port}/counter.html ⚡

    Press Ctrl+C to stop the server
    """)

    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        print("\n\n✓ Server stopped gracefully")
        httpd.shutdown()


if __name__ == '__main__':
    port = 8000
    if len(sys.argv) > 1:
        try:
            port = int(sys.argv[1])
        except ValueError:
            print(f"Invalid port number. Using default port {port}")

    run_server(port)
