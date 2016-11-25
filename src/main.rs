extern crate mio;
use mio::*;
use mio::deprecated::Handler;

struct WebSocketServer;
impl Handler for WebSocketServer{
  type Timeout = usize;
  type message = ();
}


fn main() {
    println!("Hello, world!");
}
