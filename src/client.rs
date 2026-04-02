use crate::{
    errors::{ApiError, WyzieError},
    models::{SearchParams, Subtitle},
};
use typed_builder::TypedBuilder;
use url::Url;

/// Wyzie Subs API Client
///
/// ### Example:
///
/// ```
/// use reqwest::Client;
/// use url::Url;
/// use wyzie_subs::{WyzieClient, models::SearchParams};
///
/// // Build a default client
/// let wyzie = WyzieClient::default();
///
/// // Or configure it
/// let wyzie = WyzieClient::builder()
///     .base_url(Url::parse("https://sub.wyzie.ru").unwrap())
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
    /// use reqwest::Client;
    /// use url::Url;
    /// use wyzie_subs::{WyzieClient, models::SearchParams};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let wyzie = WyzieClient::builder()
    ///         .base_url(Url::parse("https://sub.wyzie.ru").unwrap())
    ///         .reqwest_client(Client::new())
    ///         .build();
    ///
    ///     let params = SearchParams::builder()
    ///         .id("93740".to_string())
    ///         .season(1)
    ///         .episode(1)
    ///         .key(std::env::var("API_KEY").unwrap())
    ///         .build();
    ///
    ///     let subtitles = wyzie.search(&params).await.unwrap();
    /// }
    /// ```
    pub async fn search(&self, params: &SearchParams) -> Result<Vec<Subtitle>, WyzieError> {
        let url = self.base_url.join("/search")?;

        let response = self.reqwest_client.get(url).query(params).send().await?;

        if response.status().is_success() {
            let results = response.json().await?;
            Ok(results)
        } else {
            let error = response.json::<ApiError>().await?;
            Err(WyzieError::ApiError(error))
        }
    }
}
