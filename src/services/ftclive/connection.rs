use reqwest::Client;
use serde::Deserialize;
use url::Url;

use crate::services::ftclive::messages::EventDetails;

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

pub(crate) async fn get_event_details(
    url: Url,
    client: Client,
    code: String,
) -> anyhow::Result<EventDetails> {
    let mut url = url.clone();
    url.set_path(&*format!("/api/v1/events/{}", code));
    let request = client.get(url).build()?;
    let res: EventDetails = client.execute(request).await?.json().await?;
    Ok(res)
}
