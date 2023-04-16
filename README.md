# Rust Webserver
An extension over Rust's [book example](https://doc.rust-lang.org/stable/book/ch20-00-final-project-a-web-server.html).
including:
- async tcp connection handling with `mio`
- graceful shutdown on SIGINT
- thread-pool implementation