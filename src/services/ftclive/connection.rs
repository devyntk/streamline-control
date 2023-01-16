use reqwest::Client;
use serde::Deserialize;
use url::Url;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Events {
    event_codes: Vec<String>,
}

pub(crate) async fn get_event_codes(url: Url, client: Client) -> anyhow::Result<Vec<String>> {
    let mut url = url.clone();
    url.set_path("/api/v1/events/");
    let request = client.get(url).build()?;
    let res: Events = client.execute(request).await?.json().await?;
    Ok(res.event_codes)
}
