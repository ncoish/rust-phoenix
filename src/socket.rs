use std;
use std::fmt;
use std::collections::HashMap;
use std::net::TcpStream;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use tokio_core::reactor::Core;
use futures::future::Future;
use futures::sink::Sink;
use futures::stream::Stream;
use futures::sync::mpsc;

use websocket;
use websocket::ClientBuilder;
use websocket::result::WebSocketError;
//use websocket::client::sync::Client;
use websocket::{Message, OwnedMessage};

use callback;

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

pub struct WebSocket {
    sender: websocket::sender::Sender,
    receiver: websocket::receiver::Receiver,
}

// TODO: Probably want to implement builder pattern here
// TODO: Callbacks attributes should probably be list of callbacks
//       to conform to javascript client library.
pub struct Socket {
    endpoint:               String,
    //transport:              Transport,
    connected:              Arc<AtomicBool>,
    sender:                 Option<mpsc::Sender<websocket::OwnedMessage>>,
    timeout:                i32,
    state_change_open:      Option<callback::CallbackNoArg>,
    state_change_close:     Option<callback::CallbackOneArg>,
    state_change_error:     Option<callback::CallbackOneArg>,
    state_change_message:   Option<callback::CallbackOneArg>,
}

impl Socket {
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
        // if let Some(conn) = self.connection.as_mut() {
        //     return Ok(())
        // }
        if self.connected.load(Ordering::Relaxed) {
            return Ok(())
        }

        let connection_string = self.endpoint.clone();
        let (usr_msg, stdin_ch) = mpsc::channel(0);

        let connection_thread = thread::spawn(move || {
            let mut core = Core::new().unwrap();
            let runner = ClientBuilder::new(&connection_string)//(&self.endpoint[..])
                .unwrap()
                .add_protocol("rust-websocket")
                .async_connect_insecure(&core.handle())
                .and_then(|(duplex, _)| {
                    let (sink, stream) = duplex.split();
                    stream.filter_map(|message| {
                        println!("Received Message: {:?}", message);
                        match message {
                            OwnedMessage::Close(e) => Some(OwnedMessage::Close(e)),
                            OwnedMessage::Ping(d) => Some(OwnedMessage::Pong(d)),
                            _ => None,
                        }
                    })
                    .select(stdin_ch.map_err(|_| WebSocketError::NoDataAvailable))
                    .forward(sink)
                });

            core.run(runner).unwrap();
        });
        self.sender = Some(usr_msg);
        self.connected.store(true, Ordering::Relaxed);
        return Ok(())
        // let client = ClientBuilder::new(&self.endpoint[..])
        // .unwrap()
        // .connect_insecure();
        // match client {
        //     Err(e) => {
        //         Err(String::from("Failed to connect to client"))
        //     },
        //     Ok(value) => {
        //         //self.connection = Some(value);
        //         // let (mut receiver, mut sender) = value.split().unwrap();
        //         // self.connection = Some(WebSocket {
        //         //     sender: sender,
        //         //     receiver: receiver,
        //         // });
        //         // Spin up some thread here
        //         //self.processing_loop();
        //         Ok(())
        //     },
        // }
    }

    pub fn disconnect(&mut self) -> Result<(), String> {
        // Placeholder
        self.connected.store(false, Ordering::Relaxed);
        Ok(())
    }

    pub fn send(&mut self, message: String) {
        match self.sender.as_mut() {
            None => println!("Connection not established"),
            Some(value) => {
                let mut sink = value.wait();
                sink.send(OwnedMessage::Text(message));
            }
        }
    }
}

impl fmt::Debug for Socket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Socket to endpoint: {}", self.endpoint)
    }
}

#[allow(dead_code)]
pub struct SocketBuilder {
    endpoint:               String,
    //transport:              Transport,
    timeout:                i32,
    state_change_open:      Option<callback::CallbackNoArg>,
    state_change_close:     Option<callback::CallbackOneArg>,
    state_change_error:     Option<callback::CallbackOneArg>,
    state_change_message:   Option<callback::CallbackOneArg>,
}

#[allow(dead_code)]
impl SocketBuilder {
    pub fn new(endpoint: String) -> SocketBuilder {
        SocketBuilder {
            endpoint:               endpoint,
            //transport:              transport,
            timeout:                5000,
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
            timeout:                self.timeout,
            connected:              Arc::new(AtomicBool::new(false)),
            sender:                 None,
            state_change_open:      self.state_change_open,
            state_change_close:     self.state_change_close,
            state_change_error:     self.state_change_error,
            state_change_message:   self.state_change_message,
        }
    }
}
