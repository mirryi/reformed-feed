use crate::feed::document::IntoFeedEntry;
use crate::feed::state::StoredFeedEntry;
use anyhow::Result;
use rss::{ChannelBuilder, ItemBuilder};

/// Configuration for the RSS feed output.
pub struct FeedConfig {
    pub title: String,
    pub description: String,
    pub link: String,
    pub max_items: usize,
}

/// Generate RSS XML string from stored feed entries.
pub fn generate_feed(items: &[StoredFeedEntry], config: &FeedConfig) -> Result<String> {
    let rss_items: Vec<rss::Item> = items
        .iter()
        .map(|entry| {
            ItemBuilder::default()
                .title(Some(entry.title.clone()))
                .description(Some(entry.description.clone()))
                .guid(Some(rss::Guid {
                    value: entry.guid.clone(),
                    permalink: false,
                }))
                .pub_date(Some(entry.pub_date.to_rfc2822()))
                .link(Some(config.link.clone()))
                .build()
        })
        .collect();

    let channel = ChannelBuilder::default()
        .title(config.title.clone())
        .link(config.link.clone())
        .description(config.description.clone())
        .language(Some("en".to_string()))
        .last_build_date(Some(chrono::Utc::now().to_rfc2822()))
        .items(rss_items)
        .build();

    Ok(channel.to_string())
}

/// Convert any IntoFeedEntry into a StoredFeedEntry for persistence.
pub fn store_entry(entry: &dyn IntoFeedEntry) -> StoredFeedEntry {
    StoredFeedEntry {
        title: entry.title(),
        description: entry.description(),
        guid: entry.guid(),
        pub_date: chrono::Utc::now(),
    }
}
