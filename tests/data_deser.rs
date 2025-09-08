use death_stranding_poc::models::*;
use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).expect(&format!("failed to read {}", path))
}

#[test]
fn deserialize_districts() {
    let s = read("data/districts.json");
    let districts: Vec<District> = serde_json::from_str(&s).expect("districts json should parse");
    assert!(
        districts.len() >= 3,
        "Expected at least 3 districts, got {}",
        districts.len()
    );
    // ensure region matches enum
    assert!(matches!(
        districts[0].region,
        Region::West | Region::Central | Region::East
    ));
}

#[test]
fn deserialize_locations() {
    let s = read("data/locations.json");
    let locations: Vec<Location> = serde_json::from_str(&s).expect("locations json should parse");
    assert!(
        locations.len() > 36,
        "Expected at least 36 locations, got {}",
        locations.len()
    );
}

#[test]
fn deserialize_delivery_categories() {
    let s = read("data/delivery_categories.json");
    let cats: Vec<DeliveryCategory> =
        serde_json::from_str(&s).expect("delivery_categories json should parse");
    assert!(
        cats.len() >= 4,
        "Expected at least 4 delivery categories, got {}",
        cats.len()
    );
}

#[test]
fn deserialize_orders() {
    let s = read("data/orders.json");
    let orders: Vec<Order> = serde_json::from_str(&s).expect("orders json should parse");
    assert!(
        orders.len() >= 540,
        "Expected at least 540 orders, got {}",
        orders.len()
    );
}

#[test]
fn deserialize_users() {
    let s = read("data/users.json");
    let users: Vec<User> = serde_json::from_str(&s).expect("users json should parse");
    assert!(
        users.len() >= 2,
        "Expected at least 2 users, got {}",
        users.len()
    );
    assert!(
        users
            .iter()
            .all(|u| !u.name.is_empty() && !u.email.is_empty())
    );
}

#[test]
fn deserialize_deliveries() {
    let s = read("data/deliveries.json");
    let deliveries: Vec<Delivery> = serde_json::from_str(&s).expect("deliveries json should parse");
    assert!(
        deliveries.len() >= 2,
        "Expected at least 2 deliveries, got {}",
        deliveries.len()
    );
    // Ensure mapping from order_id -> order_number worked and status parsed
    assert!(deliveries.iter().all(|d| d.order_number > 0));
}
