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
use channel as phx_channel;

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

pub struct Socket {
    endpoint:               String,
    //transport:              Transport,
    channels:               Vec<Box<phx_channel::Channel>>,
    connected:              Arc<AtomicBool>,
    sender:                 Option<mpsc::Sender<websocket::OwnedMessage>>,
    timeout:                i32,
    current_ref:            Mutex<u32>,
    state_change_open:      Arc<Mutex<Vec<callback::CallbackNoArg>>>,
    state_change_close:     Arc<Mutex<Vec<callback::CallbackOneArg>>>,
    state_change_error:     Arc<Mutex<Vec<callback::CallbackOneArg>>>,
    state_change_message:   Arc<Mutex<Vec<callback::CallbackOneArg>>>,
}

impl Socket {
    // Temporary function for testing
    pub fn process_events(&mut self) {

        for func in self.state_change_open.lock().unwrap().iter_mut() {
            (func)()
        }

        for func in self.state_change_close.lock().unwrap().iter_mut() {
            (func)(String::from("closing"))
        }

        for func in self.state_change_error.lock().unwrap().iter_mut() {
            (func)(String::from("error"))
        }

        for func in self.state_change_message.lock().unwrap().iter_mut() {
            (func)(String::from("message"))
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
        //let ref_self = self.clone();

        let connection_thread = thread::spawn(move || {
            let mut core = Core::new().unwrap();
            let runner = ClientBuilder::new(&connection_string)//(&self.endpoint[..])
                .unwrap()
                .add_protocol("rust-websocket")
                .async_connect_insecure(&core.handle())
                .and_then(|(duplex, _)| {
                    //ref_self.default_on_open();
                    for func in open_lock.lock().unwrap().iter_mut() {
                        (func)();
                    }
                    let (sink, stream) = duplex.split();
                    stream.filter_map(|message| {
                        match message {
                            OwnedMessage::Close(e) => {
                                for func in close_lock.lock().unwrap().iter_mut() {
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
                                for func in message_lock.lock().unwrap().iter_mut() {
                                    (func)(text.clone());
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

    pub fn channel(&mut self, topic: String, chanParams: HashMap<String, String>) {
        let chan = phx_channel::Channel::new(topic, chanParams);
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

    fn default_on_open(&mut self) {
        println!("Hello there!");
        println!("{}", self.timeout);
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
    state_change_open:      Vec<callback::CallbackNoArg>,
    state_change_close:     Vec<callback::CallbackOneArg>,
    state_change_error:     Vec<callback::CallbackOneArg>,
    state_change_message:   Vec<callback::CallbackOneArg>,
}

#[allow(dead_code)]
impl SocketBuilder {
    pub fn new(endpoint: String) -> SocketBuilder {
        SocketBuilder {
            endpoint:               endpoint,
            //transport:              transport,
            timeout:                5000,
            state_change_open:      vec![Box::new(SocketBuilder::default_on_open)],
            state_change_close:     vec![Box::new(SocketBuilder::default_on_close)],
            state_change_error:     vec![Box::new(SocketBuilder::default_on_error)],
            state_change_message:   vec![Box::new(SocketBuilder::default_on_message)],
        }
    }

    pub fn add_on_open<CB: 'static + FnMut() + Send>(mut self, c: CB) -> SocketBuilder {
        self.state_change_open.push(Box::new(c));
        self
    }

    pub fn add_on_close<CB: 'static + FnMut(String) + Send>(mut self, c: CB) -> SocketBuilder {
        self.state_change_close.push(Box::new(c));
        self
    }

    pub fn add_on_error<CB: 'static + FnMut(String) + Send>(mut self, c: CB) -> SocketBuilder {
        self.state_change_error.push(Box::new(c));
        self
    }

    pub fn add_on_message<CB: 'static + FnMut(String) + Send>(mut self, c: CB) -> SocketBuilder {
        self.state_change_message.push(Box::new(c));
        self
    }

    fn default_on_open() {
        println!("Hello there!");
    }
    fn default_on_close(s: String) {
        println!("close: {}", s);
    }
    fn default_on_error(e: String) {
        println!("error: {}", e);
    }
    fn default_on_message(s: String) {
        println!("message: {}", s);
    }

    pub fn finish(mut self) -> Socket {
        Socket {
            endpoint:               self.endpoint,
            //transport:              self.transport,
            channels:               Vec::new(),
            connected:              Arc::new(ATOMIC_BOOL_INIT),
            sender:                 None,
            timeout:                self.timeout,
            current_ref:            Mutex::new(0),
            state_change_open:      Arc::new(Mutex::new(self.state_change_open)),
            state_change_close:     Arc::new(Mutex::new(self.state_change_close)),
            state_change_error:     Arc::new(Mutex::new(self.state_change_error)),
            state_change_message:   Arc::new(Mutex::new(self.state_change_message)),
        }
    }
    //pub fn connect(mut self) ->
}
