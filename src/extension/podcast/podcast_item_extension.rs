// This file is part of rss.
//
// Copyright © 2015-2021 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use super::{PodcastLicense, PodcastLocation, PodcastPerson};

/// This tag is used to link to a transcript or closed captions file.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[cfg_attr(
    feature = "builders",
    builder(
        setter(into),
        default,
        build_fn(name = "build_impl", private, error = "never::Never")
    )
)]
pub struct PodcastTranscript {
    /// URL of the podcast transcript.
    pub url: String,
    /// Mime type of the file such as `text/plain`, `text/html`, `text/vtt`,
    /// `application/json`, `application/x-subrip`.
    pub mime_type: String,
    /// The language of the linked transcript.
    pub language: Option<String>,
    /// Specifies the relationship between the current document and
    /// the linked document.
    pub rel: Option<String>,
}

/// Links to an external file containing chapter data for the episode.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[cfg_attr(
    feature = "builders",
    builder(
        setter(into),
        default,
        build_fn(name = "build_impl", private, error = "never::Never")
    )
)]
pub struct PodcastChapter {
    /// The URL where the chapters file is located.
    pub url: String,
    /// Mime type of file.
    pub mime_type: String,
}

/// Points to a soundbite within a podcast episode.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[cfg_attr(
    feature = "builders",
    builder(
        setter(into),
        default,
        build_fn(name = "build_impl", private, error = "never::Never")
    )
)]
pub struct PodcastSoundbite {
    /// Soundbite title.
    pub title: Option<String>,
    /// The time where the soundbite begins.
    pub start_time: f64,
    /// How long is the soundbite.
    pub duration: f64,
}

/// Identifies season the episode belongs too.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[cfg_attr(
    feature = "builders",
    builder(
        setter(into),
        default,
        build_fn(name = "build_impl", private, error = "never::Never")
    )
)]
pub struct PodcastSeason {
    /// Episode number.
    pub number: f64,
    /// The name of the season.
    pub name: Option<String>,
}

/// Identifies the episode.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[cfg_attr(
    feature = "builders",
    builder(
        setter(into),
        default,
        build_fn(name = "build_impl", private, error = "never::Never")
    )
)]
pub struct PodcastEpisode {
    /// Episode number.
    pub number: f64,
    /// The name of the season.
    pub display: Option<String>,
}

/// This element defines a uri location for a `<podcast:alternateEnclosure>` media file.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[cfg_attr(
    feature = "builders",
    builder(
        setter(into),
        default,
        build_fn(name = "build_impl", private, error = "never::Never")
    )
)]
pub struct PodcastSource {
    /// The uri where the media file resides.
    pub uri: String,
    /// This is a string that declares the mime-type of the file.
    pub content_type: Option<String>,
}

/// This element defines a method of verifying integrity of the media.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[cfg_attr(
    feature = "builders",
    builder(
        setter(into),
        default,
        build_fn(name = "build_impl", private, error = "never::Never")
    )
)]
pub struct PodcastIntegrity {
    /// Type of integrity, either "sri" or "pgp-signature".
    pub integrity_type: String,
    /// Value of the sri string or base64 encoded pgp signature.
    pub value: String,
}

/// Alternate enclosures for podcasts.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[cfg_attr(
    feature = "builders",
    builder(
        setter(into),
        default,
        build_fn(name = "build_impl", private, error = "never::Never")
    )
)]
pub struct PodcastAlternateEnclosure {
    /// Sources of this media.
    pub sources: Vec<PodcastSource>,
    /// This element defines a method of verifying integrity of the media.
    pub integrity: Option<PodcastIntegrity>,
    /// Mime type of the media asset.
    pub mime_type: String,
    /// Length of the file in bytes.
    pub length: usize,
    /// Encoding bitrate of media asset.
    pub bitrate: Option<u32>,
    /// Height of the media asset for video formats.
    pub height: Option<u32>,
    /// An IETF language tag (BCP 47) code identifying the language of this media.
    pub lang: Option<String>,
    /// A human-readable string identifying the name of the media asset.
    pub title: Option<String>,
    /// Provides a method of offering and/or grouping together different media elements.
    pub rel: Option<String>,
    /// An RFC 6381 string specifying the codecs available in this media.
    pub codecs: Option<String>,
    /// Should this enclosure be used as the default enclosure.
    pub default: bool,
}

/// A Podcast item element extension.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[cfg_attr(
    feature = "builders",
    builder(
        setter(into),
        default,
        build_fn(name = "build_impl", private, error = "never::Never")
    )
)]
pub struct PodcastItemExtension {
    /// Transcripts or closed captions file for the current item.
    pub transcript: Vec<PodcastTranscript>,
    /// A links to an external file containing chapter data for the episode.
    pub chapters: Option<PodcastChapter>,
    /// Points soundbites within a podcast episode.
    pub soundbites: Vec<PodcastSoundbite>,
    /// People of interest to the podcast.
    pub persons: Vec<PodcastPerson>,
    /// The location of editorial focus for a podcast's content
    pub location: Option<PodcastLocation>,
    /// The season the episode belongs too.
    pub season: Option<PodcastSeason>,
    /// The episode of this item.
    pub episode: Option<PodcastEpisode>,
    /// The license that is applied to the content of the item.
    pub license: Option<PodcastLicense>,
    /// Alternate enclosures for the same item.
    pub alternate_enclosures: Vec<PodcastAlternateEnclosure>,
}
