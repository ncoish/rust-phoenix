use std::collections::HashMap;
use std::collections::hash_map::Entry;

use socket::Socket;
use callback::Callback;

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
    bindings:               HashMap<ChannelEvent, Vec<Callback>>,
    timeout:                u32,
    rejoin_timer:           i32, // TODO: Need to find appropriate timer library. This should NOT be i32
    joined_once:            bool,
    join_push:              i32, // TODO: Should be Push. Still need to implement Push. This should NOT be i32
    push_buffer:            i32, // TODO: Should be Vec<Push>. Still need to implement Push. This should NOT be i32
}

impl Channel {
    pub fn new(topic: String, params: HashMap<String, String>, socket: Socket) -> Self{
        Channel {
            state: ChannelState::Closed,
            topic: topic,
            params: params,
            socket: socket,
            bindings: HashMap::with_capacity(5),
            timeout: 5000,
            rejoin_timer: 0,
            joined_once: false,
            join_push: 0,
            push_buffer: 0,
        }
    }

    fn on(&mut self, event: ChannelEvent, callback: Callback) {
        match self.bindings.entry(event) {
            Entry::Vacant(e) => { e.insert(vec![callback]); },
            Entry::Occupied(mut e) => { e.get_mut().push(callback); }
        }
    }

    fn off(&mut self, event: ChannelEvent) {
        self.bindings.remove(&event);
    }
}