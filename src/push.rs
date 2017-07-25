use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Push {
    topic:      String,
    #[serde(rename = "ref")]
    reference:  Option<String>,
    payload:    serde_json::Value,
    // Want to change this to ChannelEvent type
    event:      String,
}