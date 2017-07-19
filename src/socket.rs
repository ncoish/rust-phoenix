enum SocketState {
    Connecting,
    Open,
    Closing,
    Closed,
}

#[derive(Debug)]
pub struct Socket {

}

impl Socket {
    pub fn new(endpoint: &str, opts: Option<HashMap<String, String>>) -> Self {

    }
}
