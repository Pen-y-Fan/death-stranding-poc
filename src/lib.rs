use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
#[cfg(target_arch = "wasm32")]
use web_sys::{window, Storage};

#[derive(Serialize, Deserialize)]
struct Order {
    id: u32,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct Delivery {
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

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn initialize() {
    #[cfg(target_arch = "wasm32")]
    {
        let deliveries: Vec<Delivery> = match local_storage().get_item("deliveries").unwrap() {
            Some(value) => serde_json::from_str(&value).unwrap_or_else(|_| Vec::new()),
            None => Vec::new(),
        };
        let orders = vec![
            Order { id: 1, description: "Package A".to_string() },
            Order { id: 2, description: "Package B".to_string() },
            Order { id: 3, description: "Fragile Item".to_string() },
            Order { id: 4, description: "Important Document".to_string() },
            Order { id: 5, description: "Special Delivery".to_string() }
        ];
        let _ = local_storage().set_item("orders", &serde_json::to_string(&orders).unwrap());
        let _ = local_storage().set_item("deliveries", &serde_json::to_string(&deliveries).unwrap());
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn take_order(order_id: u32) {
    #[cfg(target_arch = "wasm32")]
    {
        let deliveries: Vec<Delivery> = match local_storage().get_item("deliveries").unwrap() {
            Some(value) => serde_json::from_str(&value).unwrap_or_else(|_| Vec::new()),
            None => Vec::new(),
        };
        let mut deliveries = deliveries;
        deliveries.push(Delivery { order_id, status: "pending".to_string() });
        let _ = local_storage().set_item("deliveries", &serde_json::to_string(&deliveries).unwrap());
        alert(&format!("Order {} taken!", order_id));
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn deliver_order(order_id: u32) {
    #[cfg(target_arch = "wasm32")]
    {
        let deliveries: Vec<Delivery> = match local_storage().get_item("deliveries").unwrap() {
            Some(value) => serde_json::from_str(&value).unwrap_or_else(|_| Vec::new()),
            None => Vec::new(),
        };
        let mut deliveries = deliveries;
        for delivery in &mut deliveries {
            if delivery.order_id == order_id {
                delivery.status = "delivered".to_string();
            }
        }
        let _ = local_storage().set_item("deliveries", &serde_json::to_string(&deliveries).unwrap());
        alert(&format!("Order {} delivered!", order_id));
    }
}

#[cfg(target_arch = "wasm32")]
fn local_storage() -> Storage {
    window().unwrap().local_storage().unwrap().unwrap()
}


