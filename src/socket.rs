use std;
use std::collections::HashMap;

enum SocketState {
    Connecting,
    Open,
    Closing,
    Closed,
}

// Probably want to implement builder pattern here
pub struct Socket {
    state_change_open:    Option<Box<FnMut()>>,
    state_change_close:   Option<Box<FnMut()>>,
    state_change_error:   Option<Box<FnMut()>>,
    state_change_message: Option<Box<FnMut()>>,
}

impl Socket {
    pub fn new(endpoint: &str) -> Self {
        Socket {
            state_change_open:    None,
            state_change_close:   None,
            state_change_error:   None,
            state_change_message: None,
        }
    }

    // pub fn new(endpoint: &str, opts: Option<HashMap<String, String>>) -> Self {

    // }

    pub fn set_callback_open<CB: 'static + FnMut()>(&mut self, c: CB) {
        self.state_change_open = Some(Box::new(c));
    }

    pub fn process_events(&mut self) {
        if let Some(ref mut func) = self.state_change_open {
            (func)()
        }
    }
}
