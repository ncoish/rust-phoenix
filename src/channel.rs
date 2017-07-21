use std::collections::HashMap;

use socket::Socket;

#[derive(Debug)]
enum ChannelState {
    Closed,
    Errored,
    Joined,
    Joining,
    Leaving,
}

impl ChannelState {
    fn value(&self) -> &str {
        match *self {
            ChannelState::Closed  => "closed",
            ChannelState::Errored => "errored",
            ChannelState::Joined  => "joined",
            ChannelState::Joining => "joining",
            ChannelState::Leaving => "leaving",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum ChannelEvent {
    Close,
    Error,
    Join,
    Reply,
    Leave,
}

impl ChannelEvent {
    fn value(&self) -> &str {
        match *self {
            ChannelEvent::Close => "phx_close",
            ChannelEvent::Error => "phx_error",
            ChannelEvent::Join  => "phx_join",
            ChannelEvent::Reply => "phx_reply",
            ChannelEvent::Leave => "phx_leave",
        }
    }
}

//#[derive(Debug)]
pub struct Channel {
    state:                  ChannelState,
    topic:                  String,
    params:                 HashMap<String, String>,
    socket:                 Socket,
    bindings:               HashMap<ChannelEvent, Vec<Box<FnMut(String)>>>,
    timeout:                u32,
    rejoin_timer:           i32, // TODO: Need to find appropriate timer library. This should NOT be i32
    joined_once:            bool,
    join_push:              i32, // TODO: Should be Push. Still need to implement Push. This should NOT be i32
    push_buffer:            i32, // TODO: Should be Vec<Push>. Still need to implement Push. This should NOT be i32
    state_change_open:      Option<Box<FnMut()>>,
    state_change_close:     Option<Box<FnMut(String)>>,
    state_change_error:     Option<Box<FnMut(String)>>,
    state_change_message:   Option<Box<FnMut(String)>>,

}

impl Channel {
    pub fn new(topic: String, params: HashMap<String, String>, socket: Socket) -> Self{
        Channel {
            state: ChannelState::Closed,
            topic: topic,
            params: params,
            socket: socket,
            bindings: HashMap::new(),
            timeout: 5000,
            rejoin_timer: 0,
            joined_once: false,
            join_push: 0,
            push_buffer: 0,
            state_change_open: None, // These should be set at initialization.
            state_change_close: None,
            state_change_error: None,
            state_change_message: None,
        }
    }

    fn on(event: ChannelEvent, FnMut)
}