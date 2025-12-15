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

        # History demo endpoints
        elif path == '/page1':
            response_html = '''
            <h2>Page 1</h2>
            <p>This is <strong>Page 1</strong> loaded with history support!</p>
            <p>The URL was pushed to history. Try clicking the back button to return to the home page.</p>
            <p>Notice that the page state is restored from the snapshot - no server request needed!</p>
            <div style="background: #e7f3ff; padding: 15px; border-radius: 4px; margin-top: 20px;">
                <strong>URL pushed:</strong> Current response URL<br>
                <strong>Scroll position:</strong> Will be restored on back navigation<br>
                <strong>Event fired:</strong> rx:history-pushed
            </div>
            '''
            self.send_html_response(response_html)

        elif path == '/page2':
            response_html = '''
            <h2>Page 2 - Custom URL</h2>
            <p>This is <strong>Page 2</strong> with a custom URL in the address bar!</p>
            <p>The browser URL is set to <code>/custom-page-2-url</code> instead of <code>/page2</code>.</p>
            <p>This demonstrates the <code>rx-push-url="/custom-url"</code> feature.</p>
            <div style="background: #fff3cd; padding: 15px; border-radius: 4px; margin-top: 20px;">
                <strong>Actual endpoint:</strong> /page2<br>
                <strong>Browser URL:</strong> /custom-page-2-url<br>
                <strong>Use case:</strong> Clean URLs for SEO
            </div>
            '''
            self.send_html_response(response_html)

        elif path == '/page3':
            response_html = '''
            <h2>Page 3 - Replace Mode</h2>
            <p>This is <strong>Page 3</strong> using <code>rx-replace-url</code>!</p>
            <p>Instead of pushing a new history entry, this <em>replaces</em> the current one.</p>
            <p>Try clicking the back button - it will skip this page and go directly to the previous page.</p>
            <div style="background: #f8d7da; padding: 15px; border-radius: 4px; margin-top: 20px;">
                <strong>Mode:</strong> Replace (not push)<br>
                <strong>Back button:</strong> Skips this page<br>
                <strong>Event fired:</strong> rx:history-replaced<br>
                <strong>Use case:</strong> Filters, tabs, or temporary states
            </div>
            '''
            self.send_html_response(response_html)

        elif path == '/page4':
            response_html = '''
            <h2>Page 4 - No Custom Title</h2>
            <p>This is <strong>Page 4</strong> without a custom title.</p>
            <p>The page title remains unchanged when navigating to this page.</p>
            <p>Only the URL is updated in the browser history.</p>
            <ul>
                <li>URL pushed to history: ✓</li>
                <li>Custom title: ✗ (keeps current title)</li>
                <li>Snapshot created: ✓</li>
                <li>Scroll restoration: ✓</li>
            </ul>
            <div style="background: #d1ecf1; padding: 15px; border-radius: 4px; margin-top: 20px;">
                <strong>Note:</strong> You can use <code>rx-push-title</code> to set a custom title.
            </div>
            '''
            self.send_html_response(response_html)

        # Trigger demo endpoints
        elif path == '/trigger/once':
            self.send_html_response(
                f'<strong style="color: #28a745;">Success!</strong> This button will not work again. (Triggered at {datetime.now().strftime("%H:%M:%S")})'
            )

        elif path == '/trigger/changed':
            form_data = urllib.parse.parse_qs(body.decode('utf-8'))
            text = form_data.get('text', [''])[0]
            self.send_html_response(
                f'<strong style="color: #28a745;">Value changed!</strong> New value: "{text}" (at {datetime.now().strftime("%H:%M:%S")})'
            )

        elif path == '/trigger/delayed':
            self.send_html_response(
                f'<strong style="color: #17a2b8;">Delayed trigger fired!</strong> You hovered 1 second ago. (at {datetime.now().strftime("%H:%M:%S")})'
            )

        elif path == '/trigger/ctrl-click':
            self.send_html_response(
                f'<strong style="color: #007bff;">Ctrl+Click detected!</strong> Filter passed. (at {datetime.now().strftime("%H:%M:%S")})'
            )

        elif path == '/trigger/shift-click':
            self.send_html_response(
                f'<strong style="color: #6c757d;">Shift+Click detected!</strong> Filter passed. (at {datetime.now().strftime("%H:%M:%S")})'
            )

        elif path == '/trigger/combo':
            self.send_html_response(
                f'<strong style="color: #ffc107;">Ctrl+Click with delay!</strong> Both filter and delay worked. (at {datetime.now().strftime("%H:%M:%S")})'
            )

        elif path == '/trigger/hover-once':
            self.send_html_response(
                f'<strong style="color: #ff6b6b;">Content loaded on first hover!</strong> This won\'t load again. (at {datetime.now().strftime("%H:%M:%S")})'
            )

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
