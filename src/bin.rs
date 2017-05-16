#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

extern crate ws;

use ws::{connect, CloseCode};

pub fn main() {
    println!("Connecting...");
    connect("ws://127.0.0.1:3012", |out| {
        out.send("Hello WebSocket").unwrap();
        out.send("Second Message").unwrap();

        move |msg| {
            println!("Got message: {}", msg);
            out.close(CloseCode::Normal)
        }
    }).unwrap();
   println!("Connection Complete!");
}
