// This file is part of rss.
//
// Copyright © 2015-2021 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

use super::PodcastPerson;

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
}
