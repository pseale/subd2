use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
struct TwitchChatMessage {
    message: String,
    user: String,
    timestamp: SystemTime,
    timestampString: String
}

fn main() {
    let timestamp = SystemTime::now();
    let dateTimeUtc : DateTime<Utc> = timestamp.into();
    let timestampString = dateTimeUtc.to_rfc3339();
    let a = TwitchChatMessage {
        message : "hey".to_string(),
        user : "jtvuser00000".to_string(),
        timestamp : timestamp,
        timestampString : timestampString
    };
    println!("{:?}",a);
    println!("{}", serde_json::to_string(&a).unwrap());
}
