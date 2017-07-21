use std;
use std::collections::HashMap;
use std::net::TcpStream;
use websocket::ClientBuilder;
use websocket::client::async::Client;

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

    pub fn process_events(&mut self) {
        if let Some(ref mut func) = self.state_change_open {
            (func)()
        }
        else {
            println!("it failed!")
        }
    }

    pub fn connect() {

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

    pub fn add_on_open<'a, CB: 'static + FnMut()>(&'a mut self, c: CB) -> &'a mut Self {
        self.state_change_open = Some(Box::new(c));
        self
    }

    pub fn add_on_close<'a, CB: 'static + FnMut(String)>(&'a mut self, c: CB) -> &'a mut Self {
        self.state_change_close = Some(Box::new(c));
        self
    }

    pub fn add_on_error<'a, CB: 'static + FnMut(String)>(&'a mut self, c: CB) -> &'a mut Self {
        self.state_change_error = Some(Box::new(c));
        self
    }

    pub fn add_on_message<'a, CB: 'static + FnMut(String)>(&'a mut self, c: CB) -> &'a mut Self {
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
