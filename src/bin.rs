#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

extern crate phoenix;

use phoenix::socket;

pub fn main() {
    let mut sock = socket::Socket::new("Hello");
    sock.set_callback_open(|| println!("Oh hello!"));
    sock.process_events();
    sock.set_callback_open(|| println!("Mello yellow"));
    sock.process_events();
}