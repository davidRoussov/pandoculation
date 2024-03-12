use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CuratedListing {
    pub items: Vec<CuratedListingItem>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CuratedListingItem {
    pub data: CuratedListingItemData,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CuratedListingItemData {
    pub title: String,
    pub url: String,
    pub author: Option<String>,
    pub id: Option<String>,
    pub points: Option<String>,
    pub timestamp: Option<String>,
    pub chat_url: Option<String>,
    #[serde(flatten)]
    pub additional: HashMap<String, String>,
}
