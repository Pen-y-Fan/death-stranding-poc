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
    #[serde(deserialize_with = "bool_from_int_or_bool")]
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
    #[serde(alias = "order_id", alias = "orderNumber", alias = "order-number")]
    pub order_number: u32,
    #[serde(deserialize_with = "delivery_status_from_str_or_enum")]
    pub status: DeliveryStatus,
    pub location_id: Option<u32>,
    pub started_at: Option<String>,
    pub ended_at: Option<String>,
    pub comment: Option<String>,
    pub user_id: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

fn delivery_status_from_str_or_enum<'de, D>(deserializer: D) -> Result<DeliveryStatus, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{Error, Unexpected};
    use serde_json::Value;

    let v = Value::deserialize(deserializer)?;
    match v {
        Value::String(s) => {
            let norm = s.to_ascii_lowercase();
            match norm.as_str() {
                "in progress" | "in_progress" | "inprogress" => Ok(DeliveryStatus::InProgress),
                "stored" => Ok(DeliveryStatus::STORED),
                "complete" | "completed" => Ok(DeliveryStatus::COMPLETE),
                "failed" | "fail" => Ok(DeliveryStatus::FAILED),
                "lost" => Ok(DeliveryStatus::LOST),
                _ => Err(Error::invalid_value(
                    Unexpected::Str(&s),
                    &"valid delivery status",
                )),
            }
        }
        Value::Object(_) | Value::Array(_) | Value::Null | Value::Number(_) => {
            // Try to deserialize as enum directly
            serde_json::from_value::<DeliveryStatus>(v).map_err(|_| {
                Error::invalid_type(Unexpected::Other("unsupported status"), &"string or enum")
            })
        }
        _ => Err(Error::invalid_type(
            Unexpected::Other("unknown"),
            &"string or enum",
        )),
    }
}

// Accepts either a JSON boolean true/false or numeric 0/1 for booleans in seed data
fn bool_from_int_or_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{Error, Unexpected};
    use serde_json::Value;

    let v = Value::deserialize(deserializer)?;
    match v {
        Value::Bool(b) => Ok(b),
        Value::Number(n) => {
            if let Some(u) = n.as_u64() {
                Ok(u != 0)
            } else if let Some(i) = n.as_i64() {
                Ok(i != 0)
            } else if let Some(f) = n.as_f64() {
                Ok(f != 0.0)
            } else {
                Err(Error::invalid_type(
                    Unexpected::Other("non-castable number"),
                    &"bool or 0/1",
                ))
            }
        }
        Value::String(s) => {
            // Accept common string variants just in case
            match s.to_ascii_lowercase().as_str() {
                "true" | "1" | "yes" | "y" => Ok(true),
                "false" | "0" | "no" | "n" => Ok(false),
                _ => Err(Error::invalid_value(Unexpected::Str(&s), &"bool or 0/1")),
            }
        }
        other => Err(Error::invalid_type(
            Unexpected::Other(match other {
                Value::Null => "null",
                Value::Array(_) => "array",
                Value::Object(_) => "object",
                _ => "unknown",
            }),
            &"bool or 0/1",
        )),
    }
}
