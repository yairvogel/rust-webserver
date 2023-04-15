use std::{io, net::SocketAddr};

use mio::{
    net::{TcpListener, TcpStream},
    Events, Interest, Poll, Token,
};

pub struct AsyncListener {
    listener: TcpListener,
    events: Events,
    poll: Poll,
}

const SOCKET_READABLE: Token = Token(0);

impl AsyncListener {
    pub fn new(address: SocketAddr) -> io::Result<AsyncListener> {
        let poll = Poll::new()?;
        let events = Events::with_capacity(32);

        let listener = register_tcp_listener(address, &poll)?;
        Ok(AsyncListener {
            poll,
            listener,
            events
        })
    }

    pub fn run<T>(&mut self, f: T) -> io::Result<()>
    where
        T: Fn(TcpStream),
    {
        loop {
            self.poll.poll(&mut self.events, None)?;
            for event in self.events.iter() {
                let stream = if let SOCKET_READABLE = event.token() {
                        Some(self.listener.accept()?.0)
                    } else { panic!("Got unexpected token") };

                if let Some(stream) = stream {
                    f(stream)
                } else {
                    return Ok(());
                };
            }
        }
    }
}

fn register_tcp_listener(address: SocketAddr, poll: &Poll) -> io::Result<TcpListener> {
    let mut listener = TcpListener::bind(address).expect("Should be able to bind to address");
    poll.registry()
        .register(&mut listener, SOCKET_READABLE, Interest::READABLE)
        .expect("Should be able to register a listener");
    Ok(listener)
}