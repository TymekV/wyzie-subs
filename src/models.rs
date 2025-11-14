use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa-impl", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "schemars-impl", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum SubtitlesFormat {
    Srt,
    Txt,
    Sub,
    Ssa,
    Ass,
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
    pub season: Option<u32>,

    /// The episode of the media if it's a show.
    pub episode: Option<u32>,

    /// The language of the subtitles that will be returned (must be in an ISO 3166-2 code).
    pub language: String,

    /// The format of subtitles returned.
    pub format: Option<SubtitlesFormat>,

    /// Determines if the subtitles are for the hearing impaired.
    pub hi: Option<bool>,

    /// The character encoding of the subtitle files.
    pub encoding: Option<String>,
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
}
