use futures::prelude::*;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

pub(crate) async fn connect_ws(base_url: Url, event_code: String) -> anyhow::Result<()> {
    let mut url = base_url.clone();
    url.set_path("/api/v2/stream");
    url.query_pairs_mut().append_pair("code", &event_code);

    let (mut ws_stream, _) = connect_async(url).await?;

    loop {
        let msg = ws_stream.next().await;
    }
    Ok(())
}
