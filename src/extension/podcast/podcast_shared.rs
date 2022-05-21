// This file is part of rss.
//
// Copyright © 2015-2021 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

/// A person of interest to the podcast.
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
pub struct PodcastPerson {
    /// Full name or alias of the person.
    pub name: String,
    /// Identifies what role the person serves on the show or episode.
    pub role: Option<String>,
    /// Reference to an official group within the Podcast Taxonomy Project list.
    pub group: Option<String>,
    /// This is the url of a picture or avatar of the person.
    pub img: Option<String>,
    /// The url to a relevant resource of information about the person.
    pub href: Option<String>,
}
