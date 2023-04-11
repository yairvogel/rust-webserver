use std::net::TcpListener;

mod threadpool;

fn main() {
    let listener = TcpListener::bind("localhost:7878")
        .expect("Failed to bind TCP listener to post 7878");
    let thread_pool = threadpool::ThreadPool::new(4).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread_pool.execute(|| webserver::handle_request(stream));
    }
}