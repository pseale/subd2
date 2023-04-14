use twitch_api::helix::HelixClient;
use twitch_api::twitch_oauth2::{AccessToken, UserToken};
use reqwest::Client as ReqwestClient;
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

#[tokio::main]
async fn main() {
    let timestamp = SystemTime::now();
    let datetime_utc : DateTime<Utc> = timestamp.into();
    let timestamp_string = datetime_utc.to_rfc3339();
    let a = TwitchChatMessage {
        message : "hey".to_string(),
        user : "jtvuser00000".to_string(),
        timestamp : timestamp,
        timestamp_string : timestamp_string
    };
    println!("{}", serde_json::to_string(&a).unwrap());
    println!("-------------------------------------");

    let oauth_token = dotenv::var("SUBD2_OAUTH_TOKEN").unwrap();
    let client: HelixClient<ReqwestClient> =  HelixClient::default();
    match UserToken::from_existing(
        &client,
        AccessToken::new(oauth_token),
        None, // Refresh Token
        None, // Client Secret
    ).await {
        Ok(response) => {
            let token = response;
            if let Ok(channel) = client.get_channel_from_login(&dotenv::var("SUBD2_USERNAME").unwrap(), &token).await {
                println!("Channel: {:?}", channel);
            }
        },
        Err(err) => panic!("{}",err)
    }
}
