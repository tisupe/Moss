# Moss

A HTTP/1.1 server written in **Rust**, built to understand networking fundamentals, sockets, request parsing, concurrency, and the HTTP protocol by implementing them from scratch.

## Features

### Core Server
- Bind to a TCP port
- Accept HTTP connections
- Respond with `200 OK`
- Extract URL path
- Read HTTP headers
- Read request body
- Return file contents

### HTTP Features
- Response body handling
- Compression negotiation (`Accept-Encoding`)
- Multiple compression schemes
- Gzip compression

### Connections
- Concurrent client connections
- Persistent (Keep-Alive) connections
- Concurrent persistent connections
- Proper connection closure
