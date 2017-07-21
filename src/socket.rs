use std;
use std::collections::HashMap;
use std::net::TcpStream;
use websocket;
use websocket::ClientBuilder;
use websocket::client::sync::Client;

enum SocketState {
    Connecting,
    Open,
    Closing,
    Closed,
}

// pub enum Transport {
//     //Longpoll(Option<i32>),
//     Websocket(Client<TcpStream>),
// }

// TODO: Probably want to implement builder pattern here
// TODO: Callbacks attributes should probably be list of callbacks
//       to conform to javascript client library.
pub struct Socket {
    endpoint:               String,
    //transport:              Transport,
    connection:             Option<Client<TcpStream>>,
    state_change_open:      Option<Box<FnMut()>>,
    state_change_close:     Option<Box<FnMut(String)>>,
    state_change_error:     Option<Box<FnMut(String)>>,
    state_change_message:   Option<Box<FnMut(String)>>,
}

impl Socket {
    pub fn set_callback_open<CB: 'static + FnMut()>(&mut self, c: CB) {
        self.state_change_open = Some(Box::new(c));
    }

    // Temporary function for testing
    pub fn process_events(&mut self) {
        if let Some(ref mut func) = self.state_change_open {
            (func)()
        }
        else {
            println!("No function set for {}", "state_change_open")
        }

        if let Some(ref mut func) = self.state_change_close {
            (func)(String::from("closing"))
        }
        else {
            println!("No function set for {}", "state_change_close")
        }

        if let Some(ref mut func) = self.state_change_error {
            (func)(String::from("error"))
        }
        else {
            println!("No function set for {}", "state_change_error")
        }

        if let Some(ref mut func) = self.state_change_message {
            (func)(String::from("message"))
        }
        else {
            println!("No function set for {}", "state_change_message")
        }
    }

    pub fn connect(&mut self) -> Result<(), String> {
        if let Some(conn) = self.connection.as_mut() {
            return Ok(())
        }
        let client = ClientBuilder::new(&self.endpoint[..])
        .unwrap()
        .connect_insecure();
        match client {
            Err(e) => {
                Err(String::from("Failed to connect to client"))
            },
            Ok(value) => {
                self.connection = Some(value);
                Ok(())
            },
        }
    }
}

#[allow(dead_code)]
pub struct SocketBuilder {
    endpoint:               String,
    //transport:              Transport,
    state_change_open:      Option<Box<FnMut()>>,
    state_change_close:     Option<Box<FnMut(String)>>,
    state_change_error:     Option<Box<FnMut(String)>>,
    state_change_message:   Option<Box<FnMut(String)>>,
}

#[allow(dead_code)]
impl SocketBuilder {
    pub fn new(endpoint: String) -> SocketBuilder {
        SocketBuilder {
            endpoint:               endpoint,
            //transport:              transport,
            state_change_open:      None,
            state_change_close:     None,
            state_change_error:     None,
            state_change_message:   None,
        }
    }

    pub fn add_on_open<CB: 'static + FnMut()>(mut self, c: CB) -> SocketBuilder {
        self.state_change_open = Some(Box::new(c));
        self
    }

    pub fn add_on_close<CB: 'static + FnMut(String)>(mut self, c: CB) -> SocketBuilder {
        self.state_change_close = Some(Box::new(c));
        self
    }

    pub fn add_on_error<CB: 'static + FnMut(String)>(mut self, c: CB) -> SocketBuilder {
        self.state_change_error = Some(Box::new(c));
        self
    }

    pub fn add_on_message<CB: 'static + FnMut(String)>(mut self, c: CB) -> SocketBuilder {
        self.state_change_message = Some(Box::new(c));
        self
    }

    pub fn finish(self) -> Socket {
        Socket {
            endpoint:               self.endpoint,
            //transport:              self.transport,
            connection:             None,
            state_change_open:      self.state_change_open,
            state_change_close:     self.state_change_close,
            state_change_error:     self.state_change_error,
            state_change_message:   self.state_change_message,
        }
    }
}
