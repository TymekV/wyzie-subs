use typed_builder::TypedBuilder;
use url::Url;

use crate::{
    errors::WyzieError,
    models::{SearchParams, Subtitle},
};

#[derive(TypedBuilder)]
pub struct WyzieClient {
    #[builder(default)]
    reqwest_client: reqwest::Client,

    #[builder(default = Url::parse("https://sub.wyzie.ru").expect("hardcoded url should parse"))]
    base_url: Url,
}

impl WyzieClient {
    pub async fn search(&self, params: &SearchParams) -> Result<Vec<Subtitle>, WyzieError> {
        let url = self.base_url.join("/search")?;

        let response = self.reqwest_client.get(url).query(params).send().await?;

        let results = response.json().await?;

        Ok(results)
    }
}
