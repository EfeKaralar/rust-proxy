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
<<<<<<< HEAD
---
The rest of the document is for design purposes, it is not fully implemented yet.
# Architecture & Technical Achievements
## Core Infrastructure:

- Asynchronous event-driven architecture using Tokio runtime with work-stealing scheduler
- Zero-copy bidirectional proxying leveraging tokio::io::copy and custom buffering strategies
- Memory-safe concurrent programming with Rust's ownership system, eliminating data races
- M:N threading model: 10,000+ lightweight tasks on 8 OS threads

## HTTP Protocol Implementation:

- Full HTTP/1.1 parsing with httparse for request/response lifecycle management
- Content-Length-based message boundary detection for precise data forwarding
- Connection keep-alive support with configurable timeouts
- Header manipulation (X-Forwarded-For, Connection management)

## Load Balancing & High Availability:

- Round-Robin: Uniform distribution with atomic counter synchronization
- Least Connections: Real-time tracking via Arc<AtomicUsize> per backend
- Weighted Round-Robin: Configurable traffic ratios for capacity-based routing
- IP Hash: Consistent session persistence using SipHash
- Active health checking with configurable intervals and retry logic
- Automatic backend pool updates without dropping in-flight requests

## Security & Performance:

- TLS 1.3 termination with rustls (memory-safe alternative to OpenSSL)
- Connection pooling with LRU eviction, reducing backend connection overhead by 60%
- Rate limiting via Tokio semaphores preventing resource exhaustion
- Graceful shutdown with connection draining

## Observability:

- Tokio Console integration for real-time task inspection and performance profiling
- Structured logging with tracing for distributed request tracing
- Prometheus-compatible metrics endpoint (requests/sec, latency percentiles, error rates)
- Per-backend health status and connection pool statistics

## Performance Characteristics:

- Throughput: 10,000 requests/second sustained
- Latency: p50: <1ms, p99: <5ms (including backend)
- Memory: ~2KB per connection (vs 2MB for thread-per-connection)
- Scalability: Linear performance scaling to 50,000+ connections

## Technical Skills Demonstrated:

- Systems programming and memory management without GC
- Async/await and Future-based concurrency
- Network socket programming (TCP, TLS)
- HTTP protocol deep dive
- Lock-free concurrent data structures
- Performance profiling and optimization
- Production systems design (health checks, circuit breakers, backpressure)
=======
- Browser establishes a TCP connection to proxy.
- Browser sends the HTTP request (with an absolute-URI) to proxy.
- Proxy establishes a TCP connection to yahoo.com (using the absolute-URI).
- Proxy forwards the HTTP request.
- Proxy receives the response.
- Proxy closes the connection to yahoo.com.
- Proxy forwards the response to browser.
- Proxy signals to close the connection (using FIN).
- Connection between browser and Proxy is closes

## RFC 9112

    A message can be either a request from client to server or a response from server to client. Syntactically, the two types of messages differ only in the start-line, which is either a request-line (for requests) or a status-line (for responses), and in the algorithm for determining the length of the message body (Section 6).

    In practice, servers are implemented to only expect a request (a response is interpreted as an unknown or invalid request method), and clients are implemented to only expect a response.

    Intermediaries that process HTTP messages (i.e., all intermediaries other than those acting as tunnels) MUST send their own HTTP-version in forwarded messages, unless it is purposefully downgraded as a workaround for an upstream issue. In other words, an intermediary is not allowed to blindly forward the start-line without ensuring that the protocol version in that message matches a version to which that intermediary is conformant for both the receiving and sending of messages. Forwarding an HTTP message without rewriting the HTTP-version might result in communication errors when downstream recipients use the message sender's version to determine what features are safe to use for later communication with that sender.

### Request Line
A request-line begins with a method token, followed by a single space (SP), the request-target, and another single space (SP), and ends with the protocol version.

  request-line   = method SP request-target SP HTTP-version

>>>>>>> 6db038a (Implement HTTPRequest parse and to_bytes method)
