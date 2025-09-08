use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::{Storage, window};

pub mod models;

// Storage keys (namespaced)
const KEY_SCHEMA_VERSION: &str = "ds:schema_version";
const KEY_DISTRICTS: &str = "ds:districts";
const KEY_LOCATIONS: &str = "ds:locations";
const KEY_CATEGORIES: &str = "ds:delivery_categories";
const KEY_ORDERS: &str = "ds:orders";
const KEY_DELIVERIES: &str = "ds:deliveries";

#[derive(Serialize, Deserialize)]
struct DemoOrder {
    id: u32,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct DemoDelivery {
    order_id: u32,
    status: String, // e.g., "pending", "in_transit", "delivered"
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str); // For basic browser alerts (for debugging)
}
#[cfg(not(target_arch = "wasm32"))]
fn alert(_s: &str) {}

// ---------- WASM boundary: Data loading and persistence ----------
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn set_schema_version(version: &str) -> Result<(), String> {
    #[cfg(target_arch = "wasm32")]
    {
        local_storage()
            .set_item(KEY_SCHEMA_VERSION, version)
            .map_err(|_| "failed to set schema_version".to_string())?;
        return Ok(());
    }
    #[allow(unreachable_code)]
    Ok(())
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn get_schema_version() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        return local_storage()
            .get_item(KEY_SCHEMA_VERSION)
            .ok()
            .flatten()
            .unwrap_or_default();
    }
    String::new()
}

fn parse_json_vec<T: for<'de> Deserialize<'de>>(json: &str) -> Result<Vec<T>, String> {
    serde_json::from_str::<Vec<T>>(json).map_err(|e| format!("invalid JSON array: {}", e))
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn import_districts(json: &str) -> Result<(), String> {
    let districts: Vec<models::District> = parse_json_vec(json)?;
    #[cfg(target_arch = "wasm32")]
    {
        local_storage()
            .set_item(KEY_DISTRICTS, &serde_json::to_string(&districts).unwrap())
            .map_err(|_| "failed to persist districts".to_string())?;
        let _ = set_schema_version("1");
    }
    Ok(())
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn import_locations(json: &str) -> Result<(), String> {
    let locations: Vec<models::Location> = parse_json_vec(json)?;
    #[cfg(target_arch = "wasm32")]
    {
        local_storage()
            .set_item(KEY_LOCATIONS, &serde_json::to_string(&locations).unwrap())
            .map_err(|_| "failed to persist locations".to_string())?;
        let _ = set_schema_version("1");
    }
    Ok(())
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn import_delivery_categories(json: &str) -> Result<(), String> {
    let cats: Vec<models::DeliveryCategory> = parse_json_vec(json)?;
    #[cfg(target_arch = "wasm32")]
    {
        local_storage()
            .set_item(KEY_CATEGORIES, &serde_json::to_string(&cats).unwrap())
            .map_err(|_| "failed to persist delivery_categories".to_string())?;
        let _ = set_schema_version("1");
    }
    Ok(())
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn import_orders(json: &str) -> Result<(), String> {
    let orders: Vec<models::Order> = parse_json_vec(json)?;
    // Validate constraints (unique number, fields)
    models::validate_orders(&orders)?;
    #[cfg(target_arch = "wasm32")]
    {
        local_storage()
            .set_item(KEY_ORDERS, &serde_json::to_string(&orders).unwrap())
            .map_err(|_| "failed to persist orders".to_string())?;
        let _ = set_schema_version("1");
    }
    Ok(())
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn get_districts() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        return local_storage()
            .get_item(KEY_DISTRICTS)
            .ok()
            .flatten()
            .unwrap_or_else(|| "[]".into());
    }
    "[]".into()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn get_locations() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        return local_storage()
            .get_item(KEY_LOCATIONS)
            .ok()
            .flatten()
            .unwrap_or_else(|| "[]".into());
    }
    "[]".into()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn get_delivery_categories() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        return local_storage()
            .get_item(KEY_CATEGORIES)
            .ok()
            .flatten()
            .unwrap_or_else(|| "[]".into());
    }
    "[]".into()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn get_orders() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        return local_storage()
            .get_item(KEY_ORDERS)
            .ok()
            .flatten()
            .unwrap_or_else(|| "[]".into());
    }
    "[]".into()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn import_deliveries(json: &str) -> Result<(), String> {
    let deliveries: Vec<models::Delivery> = parse_json_vec(json)?;
    #[cfg(target_arch = "wasm32")]
    {
        local_storage()
            .set_item(KEY_DELIVERIES, &serde_json::to_string(&deliveries).unwrap())
            .map_err(|_| "failed to persist deliveries".to_string())?;
    }
    Ok(())
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn export_deliveries() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        return local_storage()
            .get_item(KEY_DELIVERIES)
            .ok()
            .flatten()
            .unwrap_or_else(|| "[]".into());
    }
    "[]".into()
}

// ---------- Existing demo functions (kept for compatibility) ----------
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn initialize() {
    #[cfg(target_arch = "wasm32")]
    {
        let deliveries: Vec<DemoDelivery> = match local_storage().get_item(KEY_DELIVERIES).unwrap()
        {
            Some(value) => serde_json::from_str(&value).unwrap_or_else(|_| Vec::new()),
            None => Vec::new(),
        };
        let orders = vec![
            DemoOrder {
                id: 1,
                description: "Package A".to_string(),
            },
            DemoOrder {
                id: 2,
                description: "Package B".to_string(),
            },
            DemoOrder {
                id: 3,
                description: "Fragile Item".to_string(),
            },
            DemoOrder {
                id: 4,
                description: "Important Document".to_string(),
            },
            DemoOrder {
                id: 5,
                description: "Special Delivery".to_string(),
            },
        ];
        let _ = local_storage().set_item(KEY_ORDERS, &serde_json::to_string(&orders).unwrap());
        let _ =
            local_storage().set_item(KEY_DELIVERIES, &serde_json::to_string(&deliveries).unwrap());
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn take_order(order_id: u32) {
    #[cfg(target_arch = "wasm32")]
    {
        let deliveries: Vec<DemoDelivery> = match local_storage().get_item(KEY_DELIVERIES).unwrap()
        {
            Some(value) => serde_json::from_str(&value).unwrap_or_else(|_| Vec::new()),
            None => Vec::new(),
        };
        let mut deliveries = deliveries;
        deliveries.push(DemoDelivery {
            order_id,
            status: "pending".to_string(),
        });
        let _ =
            local_storage().set_item(KEY_DELIVERIES, &serde_json::to_string(&deliveries).unwrap());
        alert(&format!("Order {} taken!", order_id));
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn deliver_order(order_id: u32) {
    #[cfg(target_arch = "wasm32")]
    {
        let deliveries: Vec<DemoDelivery> = match local_storage().get_item(KEY_DELIVERIES).unwrap()
        {
            Some(value) => serde_json::from_str(&value).unwrap_or_else(|_| Vec::new()),
            None => Vec::new(),
        };
        let mut deliveries = deliveries;
        for delivery in &mut deliveries {
            if delivery.order_id == order_id {
                delivery.status = "delivered".to_string();
            }
        }
        let _ =
            local_storage().set_item(KEY_DELIVERIES, &serde_json::to_string(&deliveries).unwrap());
        alert(&format!("Order {} delivered!", order_id));
    }
}

#[cfg(target_arch = "wasm32")]
fn local_storage() -> Storage {
    window().unwrap().local_storage().unwrap().unwrap()
}
