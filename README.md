# Rust Proxy
A simple network proxy written in Rust as an exercise.

# Challenge
To build a simple network proxy and load balancer for personal use + education

# What's done so for
- [x] Pure TCP Proxy
- [ ] HTTP Aware Proxy
- [ ] Load Balancer
- [ ] Health Checks
- [ ] TLS Termination
- [ ] Additional Features

# Notes
## HTTP 
- HTTP Request 
`
GET /path HTTP/1.1\r\n
Host: example.com\r\n
User-Agent: curl/7.64.1\r\n
Connection: keep-alive\r\n
\r\n
[optional body]
`
- HTTP Response
`
HTTP/1.1 200 OK\r\n
Content-Type: text/plain\r\n
Content-Length: 13\r\n
Connection: close\r\n
\r\n
Hello, World!
`
