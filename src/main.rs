use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
struct TwitchChatMessage {
    message: String,
    user: String,
    timestamp: SystemTime,
    timestamp_string: String
}

fn main() {
    let timestamp = SystemTime::now();
    let datetime_utc : DateTime<Utc> = timestamp.into();
    let timestamp_string = datetime_utc.to_rfc3339();
    let a = TwitchChatMessage {
        message : "hey".to_string(),
        user : "jtvuser00000".to_string(),
        timestamp : timestamp,
        timestamp_string : timestamp_string
    };
    println!("env var is: {}", dotenv::var("TEMP").unwrap());
    println!("{}", serde_json::to_string(&a).unwrap());
}
