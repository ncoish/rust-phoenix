use std;
use std::fmt;
use std::collections::HashMap;
use std::net::TcpStream;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT, Ordering};
use std::sync::Mutex;

use tokio_core::reactor::Core;
use futures::future::Future;
use futures::sink::Sink;
use futures::stream::Stream;
use futures::sync::mpsc;

use serde_json;
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

// TODO: Callbacks attributes should probably be list of callbacks
//       to conform to javascript client library.
pub struct Socket {
    endpoint:               String,
    //transport:              Transport,
    connected:              Arc<AtomicBool>,
    sender:                 Option<mpsc::Sender<websocket::OwnedMessage>>,
    timeout:                i32,
    current_ref:            Mutex<u32>,
    state_change_open:      Arc<Mutex<Option<callback::CallbackNoArg>>>,
    state_change_close:     Arc<Mutex<Option<callback::CallbackOneArg>>>,
    state_change_error:     Arc<Mutex<Option<callback::CallbackOneArg>>>,
    state_change_message:   Arc<Mutex<Option<callback::CallbackOneArg>>>,
}

impl Socket {
    // Temporary function for testing
    pub fn process_events(&mut self) {
        let mut internal = self.state_change_open.lock().unwrap();
        if let Some(ref mut func) = *internal {
            (func)()
        }
        else {
            println!("No function set for {}", "state_change_open")
        }
        let mut internal = self.state_change_close.lock().unwrap();
        if let Some(ref mut func) = *internal {
            (func)(String::from("closing"))
        }
        else {
            println!("No function set for {}", "state_change_close")
        }

        let mut internal = self.state_change_error.lock().unwrap();
        if let Some(ref mut func) = *internal {
            (func)(String::from("error"))
        }
        else {
            println!("No function set for {}", "state_change_error")
        }

        let mut internal = self.state_change_message.lock().unwrap();
        if let Some(ref mut func) = *internal {
            (func)(String::from("message"))
        }
        else {
            println!("No function set for {}", "state_change_message")
        }
    }

    pub fn connect(&mut self) -> Result<(), String> {
        if self.connected.load(Ordering::Relaxed) {
            return Ok(())
        }

        let connection_string = self.endpoint.clone();
        let open_lock = self.state_change_open.clone();
        let close_lock = self.state_change_close.clone();
        let error_lock = self.state_change_error.clone();
        let message_lock = self.state_change_message.clone();

        let (usr_msg, stdin_ch) = mpsc::channel(0);

        let connection_thread = thread::spawn(move || {
            let mut core = Core::new().unwrap();
            let runner = ClientBuilder::new(&connection_string)//(&self.endpoint[..])
                .unwrap()
                .add_protocol("rust-websocket")
                .async_connect_insecure(&core.handle())
                .and_then(|(duplex, _)| {
                    let mut internal = open_lock.lock().unwrap();
                    if let Some(ref mut func) = *internal {
                        (func)();
                    }
                    let (sink, stream) = duplex.split();
                    stream.filter_map(|message| {
                        match message {
                            OwnedMessage::Close(e) => {
                                let mut internal = close_lock.lock().unwrap();
                                if let Some(ref mut func) = *internal {
                                    (func)(e.clone().unwrap().reason);
                                }
                                Some(OwnedMessage::Close(e))
                            },
                            OwnedMessage::Ping(d) => {
                                println!("Got a ping!");
                                Some(OwnedMessage::Pong(d))
                            },
                            OwnedMessage::Pong(d) => {
                                println!("Got a pong for some reason");
                                None
                            }
                            OwnedMessage::Text(text) => {
                                let mut internal = message_lock.lock().unwrap();
                                if let Some(ref mut func) = *internal {
                                    (func)(text);
                                }
                                None
                            },
                            OwnedMessage::Binary(bin) => {
                                println!("binary: {:?}", bin);
                                None
                            },

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

    fn make_ref(&mut self) -> u32 {
        let mut ref_num = self.current_ref.lock().unwrap();
        let return_ref = *ref_num;
        *ref_num += 1;
        return return_ref
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

    pub fn add_on_open<CB: 'static + FnMut() + Send>(mut self, c: CB) -> SocketBuilder {
        self.state_change_open = Some(Box::new(c));
        self
    }

    pub fn add_on_close<CB: 'static + FnMut(String) + Send>(mut self, c: CB) -> SocketBuilder {
        self.state_change_close = Some(Box::new(c));
        self
    }

    pub fn add_on_error<CB: 'static + FnMut(String) + Send>(mut self, c: CB) -> SocketBuilder {
        self.state_change_error = Some(Box::new(c));
        self
    }

    pub fn add_on_message<CB: 'static + FnMut(String) + Send>(mut self, c: CB) -> SocketBuilder {
        self.state_change_message = Some(Box::new(c));
        self
    }

    pub fn finish(self) -> Socket {
        Socket {
            endpoint:               self.endpoint,
            //transport:              self.transport,
            timeout:                self.timeout,
            connected:              Arc::new(ATOMIC_BOOL_INIT),
            sender:                 None,
            current_ref:            Mutex::new(0),
            state_change_open:      Arc::new(Mutex::new(self.state_change_open)),
            state_change_close:     Arc::new(Mutex::new(self.state_change_close)),
            state_change_error:     Arc::new(Mutex::new(self.state_change_error)),
            state_change_message:   Arc::new(Mutex::new(self.state_change_message)),
        }
    }
}
