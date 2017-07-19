enum ChannelStates {
    Closed,
    Errored,
    Joined,
    Joining,
    Leaving,
}

impl ChannelStates {
    fn value(&self) -> &str {
        match *self {
            ChannelStates::Closed  => "closed",
            ChannelStates::Errored => "errored",
            ChannelStates::Joined  => "joined",
            ChannelStates::Joining => "joining",
            ChannelStates::Leaving => "leaving",
        }
    }
}

enum ChannelEvents {
    Close,
    Error,
    Join,
    Reply,
    Leave,
}

impl ChannelEvents {
    fn value(&self) -> &str {
        match *self {
            ChannelEvents::Close => "phx_close",
            ChannelEvents::Error => "phx_error",
            ChannelEvents::Join  => "phx_join",
            ChannelEvents::Reply => "phx_reply",
            ChannelEvents::Leave => "phx_leave",
        }
    }
}

#[derive(Debug)]
pub struct Channel {

}
