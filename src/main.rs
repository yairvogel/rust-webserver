
mod threadpool;
mod async_listener;

use std::{
    net::{SocketAddr, IpAddr, Ipv4Addr}, io::ErrorKind
};
use threadpool::ThreadPool;

fn main() {

    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 7878);
    let thread_pool = ThreadPool::new(4).unwrap();

    let mut listener = async_listener::AsyncListener::new(address)
        .expect("Should be able to init listener");

    // disabling SIGINT
    ctrlc::set_handler(|| {})
        .expect("Should be able to set handler");

    // if let Err(error) = 
    if let Err(err) = listener.run(|stream| thread_pool.execute(|| webserver::handle_request(stream))) {
        if err.kind() != ErrorKind::Interrupted {
            panic!("{err:?}")
        }
    }
}