use anyhow::Result;

/// Converts an item into data suitable for an RSS feed entry.
pub trait IntoFeedEntry {
    fn title(&self) -> String;
    fn description(&self) -> String;
    /// Unique identifier for deduplication and state tracking.
    fn guid(&self) -> String;
}

/// A parsed document containing a sequence of items.
pub trait Document {
    type Item: IntoFeedEntry;

    /// Returns all items in document order.
    fn items(&self) -> Vec<Self::Item>;
    /// Document identifier (e.g. "westminster-shorter-catechism").
    fn id(&self) -> &str;
    /// Human-readable title.
    fn title(&self) -> &str;
}

/// A parsing strategy that produces a Document from raw data.
pub trait ParseStrategy {
    type Doc: Document;
    fn parse(&self, data: &[u8]) -> Result<Self::Doc>;
}
