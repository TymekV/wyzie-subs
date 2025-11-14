use typed_builder::TypedBuilder;
use url::Url;

use crate::{
    errors::WyzieError,
    models::{SearchParams, Subtitle},
};

/// Wyzie Subs API Client
///
/// ### Example:
///
/// ```
/// // Build a default client
/// let wyzie = WyzieClient::default();
///
/// // Or configure it
/// let wyzie = WyzieClient::builder()
///     .base_url(Url::parse("https://sub.wyzie.ru")?)
///     .reqwest_client(Client::new())
///     .build();
/// ```
#[derive(TypedBuilder, Clone, Debug)]
pub struct WyzieClient {
    #[builder(default)]
    reqwest_client: reqwest::Client,

    #[builder(default = Url::parse("https://sub.wyzie.ru").expect("hardcoded url should parse"))]
    base_url: Url,
}

impl Default for WyzieClient {
    fn default() -> Self {
        WyzieClient::builder().build()
    }
}

impl WyzieClient {
    /// Search for subtitles
    ///
    /// ### Example:
    /// ```
    /// let params = SearchParams::builder()
    ///     .id("93740".to_string())
    ///     .season(1)
    ///     .episode(1)
    ///     .build();
    ///
    /// let subtitles = wyzie.search(&params).await?;
    /// ```
    pub async fn search(&self, params: &SearchParams) -> Result<Vec<Subtitle>, WyzieError> {
        let url = self.base_url.join("/search")?;

        let response = self.reqwest_client.get(url).query(params).send().await?;

        let results = response.json().await?;

        Ok(results)
    }
}
