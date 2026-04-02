use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, EnumString, Display)]
#[cfg_attr(feature = "utoipa-impl", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "schemars-impl", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase", untagged)]
#[strum(serialize_all = "lowercase")]
pub enum SubtitlesFormat {
    Srt,
    Txt,
    Sub,
    Ssa,
    Ass,
    Other(String),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, EnumString, Display)]
#[cfg_attr(feature = "utoipa-impl", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "schemars-impl", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase", untagged)]
#[strum(serialize_all = "lowercase")]
pub enum SubtitlesSource {
    All,
    Subdl,
    Sub2Fm,
    OpenSubtitles,
    Podnapisi,
    AnimeTosho,
    Gestdown,
    Jimaku,
    Kitsunekko,
    Yify,
    AjatTools,
    Other(String),
}

#[derive(Serialize, TypedBuilder, Debug)]
#[cfg_attr(feature = "utoipa-impl", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "schemars-impl", derive(schemars::JsonSchema))]
pub struct SearchParams {
    /// The TMDB or IMDB ID of the show or movie
    ///
    /// > [!INFO]
    /// > When using an IMDB ID, ensure that the first two characters (`tt`) are included at the beginning of the ID.
    pub id: String,

    /// The season of the media if it's a show.
    #[builder(default, setter(strip_option))]
    pub season: Option<u32>,

    /// The episode of the media if it's a show.
    #[builder(default, setter(strip_option))]
    pub episode: Option<u32>,

    /// The language of the subtitles that will be returned (must be an ISO 3166-2 code).
    #[builder(default, setter(strip_option))]
    pub language: Option<String>,

    /// The format of subtitles returned.
    #[serde(with = "serde_qs::helpers::comma_separated")]
    #[builder(default)]
    pub format: Vec<SubtitlesFormat>,

    /// Determines if the subtitles are for the hearing impaired.
    #[builder(default, setter(strip_option))]
    pub hi: Option<bool>,

    /// The character encoding of the subtitle files.
    #[builder(default, setter(strip_option))]
    pub encoding: Option<String>,

    /// Subtitle providers to query (`all` queries every enabled source; default `opensubtitles`).
    #[serde(with = "serde_qs::helpers::comma_separated")]
    #[builder(default)]
    pub source: Vec<SubtitlesSource>,

    /// Release or scene name filters.
    #[serde(with = "serde_qs::helpers::comma_separated")]
    #[builder(default)]
    pub release: Vec<String>,

    /// Filename filters.
    #[serde(with = "serde_qs::helpers::comma_separated")]
    #[builder(default)]
    pub filename: Vec<String>,

    /// Content origin filter (e.g. `WEB`, `BLURAY`, `DVD`).
    #[serde(with = "serde_qs::helpers::comma_separated")]
    #[builder(default)]
    pub origin: Vec<String>,

    /// Your API key (required). Get one free at sub.wyzie.io/redeem.
    pub key: String,

    /// Bypass cache and fetch fresh results. Use when sources may have updated.
    #[builder(default, setter(strip_option))]
    pub refresh: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa-impl", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "schemars-impl", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct Subtitle {
    /// The ID of the subtitle file.
    pub id: String,

    /// The URL to the subtitle file.
    pub url: String,

    /// URL to the flag of the language's locale.
    pub flag_url: String,

    /// The language of the subtitle file.
    pub format: SubtitlesFormat,

    /// The character encoding of the subtitle file.
    pub encoding: String,

    /// The language of the subtitle, capitalized.
    pub display: String,

    /// The ISO 3166-2 code of the language.
    pub language: String,

    /// The name of the media that the subtitles are for.
    pub media: String,

    /// Boolean representing if the subtitle is hearing impaired accessible.
    pub is_hearing_impaired: bool,

    /// Which source the subtitle was scraped from.
    pub source: String,

    /// Primary release name.
    pub release: Option<String>,

    /// Other release names compatible with the subtitle.
    pub releases: Vec<String>,

    /// Original filename when available.
    pub file_name: Option<String>,

    /// Number of downloads on the source platform (if available).
    pub download_count: Option<u32>,

    /// Content origin (e.g., WEB, BluRay, DVD).
    pub origin: Option<String>,

    /// Release value that matched your filter (if provided).
    pub matched_release: Option<String>,

    /// The user-supplied filter that matched (if provided).
    pub matched_filter: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_serailzes() {
        assert_eq!(
            SubtitlesSource::OpenSubtitles.to_string(),
            "opensubtitles".to_string()
        );
    }

    #[test]
    fn params_serialize() {
        let params = SearchParams::builder()
            .id("tt3659388".into())
            .source(vec![
                SubtitlesSource::Subdl,
                SubtitlesSource::OpenSubtitles,
                SubtitlesSource::Other("custom".into()),
            ])
            .release(vec!["a".into(), "b".into()])
            .key("key".to_string())
            .build();

        let serialized = serde_urlencoded::to_string(params).unwrap();

        assert_eq!(serialized, "id=tt3659388&format=&source=subdl%2Copensubtitles%2Cother&release=a%2Cb&filename=&origin=&key=key".to_string());
    }
}
