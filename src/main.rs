use reqwest::Client as ReqwestClient;
use reqwest::Url;
use tokio_tungstenite::tungstenite;
use twitch_api::helix::HelixClient;
use twitch_api::tmi::UserId;
use twitch_api::twitch_oauth2::{AccessToken, UserToken};

async fn open_websocket(
    oauth_token: String,
    client: HelixClient<'static, ReqwestClient>,
    broadcaster_id: u64,
    connect_url: Url,
) -> Result<
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    eyre::Error,
> {
    unimplemented!();
    //       let config = tungstenite::protocol::WebSocketConfig {
    //             max_send_queue: None,
    //             max_message_size: Some(64 << 20), // 64 MiB
    //             max_frame_size: Some(16 << 20),   // 16 MiB
    //             accept_unmasked_frames: false,
    //         };
    //         let (socket, _) =
    //             tokio_tungstenite::connect_async_with_config(&self.connect_url, Some(config))
    //                 .await
    //                 .context("Can't connect")?;
    //
    //         Ok(socket)
}
#[tokio::main]
async fn main() {
    let oauth_token = dotenv::var("SUBD2_OAUTH_TOKEN").unwrap();
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
                .get_channel_from_login(&dotenv::var("SUBD2_USERNAME").unwrap(), &token)
                .await
            {
                println!("Channel: {:?}", channel);

                let connect_url = twitch_api::TWITCH_EVENTSUB_WEBSOCKET_URL.clone();
                let broadcaster_id = 792462754;
                let socket = open_websocket(oauth_token, client, broadcaster_id, connect_url);
            }
        }
        Err(err) => panic!("{}", err),
    }
}
