#!/usr/bin/env python3
from http.server import BaseHTTPRequestHandler, HTTPServer

"""
This simple server prints received messages and methods to standard output, but only for the accepted path.

We can register this webhook to the netspot_control using the JSON below:

{
  "name": "Our example server",
  "address": "http://localhost:9001/webhook",
  "method": "POST",
  "type": "both"
}

We may need to edit the configuration to match our setup.

Use the variables below to configure the server.
"""

SERVER_ADDRESS = ''
SERVER_PORT = 9001
ACCEPTED_PATH = '/webhook'


# Simple server that prints received messages
class Webhook(BaseHTTPRequestHandler):

    # Common message handler for GET, POST and PUT requests.
    def message_handler(self, method):
        if self.path == ACCEPTED_PATH:
            if 'Content-Length' in self.headers:
                content_length = int(self.headers['Content-Length'])
                message = self.rfile.read(content_length).decode('utf-8')
                print(f'{method}: {message}')
            else:
                print(f'{method}: No message')
            self.send_response(200)
            self.end_headers()
            # print(self.headers)  # Enable this if you want to see headers
        else:
            print("Responding with 404 to an unknown path:", self.path)
            self.send_response(404)
            self.send_header('Content-type', 'text/html')
            self.end_headers()
            self.wfile.write('<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><title>404 Not Found</title>'
                             '</head><body><h1>404: Not Found</h1><p>The requested resource could not be found.</p>'
                             '</body></html>'.encode('utf-8'))

    def do_GET(self):
        self.message_handler('GET')

    def do_POST(self):
        self.message_handler('POST')

    def do_PUT(self):
        self.message_handler('PUT')

    def log_message(self, format, *args):
        # Silencing log messages to print only messages from netspots
        pass


def main():
    httpd = HTTPServer((SERVER_ADDRESS, SERVER_PORT), Webhook)
    print('Webhook server started...')
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        pass
    httpd.server_close()
    print('Webhook server stopped...')


if __name__ == '__main__':
    main()
