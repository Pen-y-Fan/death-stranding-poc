use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Region {
    East,
    Central,
    West,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct District {
    pub id: u32,
    pub name: String,
    pub region: Region,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Location {
    pub id: u32,
    pub name: String,
    pub district_id: u32,
    pub is_physical: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeliveryCategory {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Order {
    /// Unique order number
    pub number: u32,
    pub name: String,
    /// Location id
    pub client_id: u32,
    /// Location id
    pub destination_id: u32,
    pub delivery_category_id: u32,
    pub max_likes: f32,
    pub weight: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeliveryStatus {
    InProgress, // Was IN_PROGRESS
    STORED,
    COMPLETE,
    FAILED,
    LOST,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Delivery {
    pub id: u32,
    pub order_number: u32,
    pub status: DeliveryStatus,
    pub location_id: Option<u32>,
    pub started_at: Option<String>,
    pub ended_at: Option<String>,
    pub comment: Option<String>,
    pub user_id: Option<u32>,
}
