use serde_json;

use std::sync::{Weak};
use channel as phx_channel;

pub struct Push {
    channel:      Weak<phx_channel::Channel>,
    reference:  Option<String>,
    payload:    serde_json::Value,
    // Want to change this to ChannelEvent type
    event:      String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PushObject {
    topic:      String,
    #[serde(rename = "ref")]
    reference:  Option<String>,
    payload:    serde_json::Value,
    // Want to change this to ChannelEvent type
    event:      String,
}
