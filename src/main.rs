extern crate mio;
use mio::*;
use mio::deprecated::Handler;

struct WebSocketServer;
impl Handler for WebSocketServer{
  type Timeout = usize;
  type Message = ();
}


fn main() {
    let mut event_loop = EventLoop::new().unwrap();

    //creates new instance of Handler
    let mut handler = WebSocketServer;
    event_loop.run(&mut handler).unwrap();

    
    println!("Hello, world!");
}
