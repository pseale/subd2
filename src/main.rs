use eyre::Context;
use reqwest::Client as ReqwestClient;
use reqwest::Url;
use tokio_tungstenite::tungstenite;
use tungstenite::protocol::WebSocketConfig;
use twitch_api::helix::HelixClient;
use twitch_api::twitch_oauth2::{AccessToken, UserToken};

async fn open_websocket(
    connect_url: Url,
) -> Result<
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    eyre::Error,
> {
    let config = WebSocketConfig {
        max_send_queue: None,
        max_message_size: Some(64 << 20), // 64 MiB
        max_frame_size: Some(16 << 20),   // 16 MiB
        accept_unmasked_frames: false,
    };
    let (socket, _) = tokio_tungstenite::connect_async_with_config(connect_url, Some(config))
        .await
        .context("Can't connect")?;

    Ok(socket)
}
#[tokio::main]
async fn main() {
    let oauth_token = dotenv::var("SUBD2_BROADCASTER_OAUTH_TOKEN").unwrap();
    let client: HelixClient<ReqwestClient> = HelixClient::default();
    match UserToken::from_existing(
        &client,
        AccessToken::new(oauth_token.clone()),
        None, // Refresh Token
        None, // Client Secret
    )
    .await
    {
        Ok(response) => {
            let token = response;
            if let Ok(Some(channel)) = client
                .get_channel_from_login(&dotenv::var("SUBD2_BROADCASTER_USERNAME").unwrap(), &token)
                .await
            {
                println!("Channel: {:?}", channel);

                let connect_url = twitch_api::TWITCH_EVENTSUB_WEBSOCKET_URL.clone();
                let socket = open_websocket(connect_url).await;
            }
        }
        Err(err) => panic!("{}", err),
    }
}
