// This file is part of rss.
//
// Copyright © 2015-2021 The rust-syndication Developers
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the MIT License and/or Apache 2.0 License.

mod podcast_channel_extension;
mod podcast_item_extension;
mod podcast_shared;

pub use self::podcast_channel_extension::*;
pub use self::podcast_item_extension::*;
pub use self::podcast_shared::*;

/// The Podcast XML namespace.
pub const NAMESPACE: &str = "https://podcastindex.org/namespace/1.0";
