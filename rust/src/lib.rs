use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CuratedListing {
    items: Vec<CuratedListingItem>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CuratedListingItem {
    data: CuratedListingItemData,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CuratedListingItemData {
    title: String,
    url: String,
    author: Option<String>,
    id: Option<String>,
    points: Option<String>,
    timestamp: Option<String>,
    chat_url: Option<String>,
    #[serde(flatten)]
    additional: HashMap<String, String>,
}
