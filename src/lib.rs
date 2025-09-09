use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::{Storage, window};

pub mod models;
pub mod services;

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
        let storage = safe_local_storage()?;
        storage
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
        return match safe_local_storage() {
            Ok(s) => s
                .get_item(KEY_SCHEMA_VERSION)
                .ok()
                .flatten()
                .unwrap_or_default(),
            Err(_) => String::new(),
        };
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
        safe_local_storage()?
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
        safe_local_storage()?
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
        safe_local_storage()?
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
        safe_local_storage()?
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
        return match safe_local_storage() {
            Ok(s) => s
                .get_item(KEY_DISTRICTS)
                .ok()
                .flatten()
                .unwrap_or_else(|| "[]".into()),
            Err(_) => "[]".into(),
        };
    }
    "[]".into()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn get_locations() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        return match safe_local_storage() {
            Ok(s) => s
                .get_item(KEY_LOCATIONS)
                .ok()
                .flatten()
                .unwrap_or_else(|| "[]".into()),
            Err(_) => "[]".into(),
        };
    }
    "[]".into()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn get_location(id: u32) -> String {
    #[cfg(target_arch = "wasm32")]
    {
        if let Ok(storage) = safe_local_storage() {
            if let Ok(Some(s)) = storage.get_item(KEY_LOCATIONS) {
                if let Ok(list) = serde_json::from_str::<Vec<models::Location>>(&s) {
                    if let Some(loc) = list.into_iter().find(|l| l.id == id) {
                        return serde_json::to_string(&loc).unwrap_or_else(|_| "{}".into());
                    }
                }
            }
        }
        return "{}".into();
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        return "{}".into();
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn get_district(id: u32) -> String {
    #[cfg(target_arch = "wasm32")]
    {
        if let Ok(storage) = safe_local_storage() {
            if let Ok(Some(s)) = storage.get_item(KEY_DISTRICTS) {
                if let Ok(list) = serde_json::from_str::<Vec<models::District>>(&s) {
                    if let Some(d) = list.into_iter().find(|d| d.id == id) {
                        return serde_json::to_string(&d).unwrap_or_else(|_| "{}".into());
                    }
                }
            }
        }
        return "{}".into();
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        return "{}".into();
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn get_delivery_categories() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        return match safe_local_storage() {
            Ok(s) => s
                .get_item(KEY_CATEGORIES)
                .ok()
                .flatten()
                .unwrap_or_else(|| "[]".into()),
            Err(_) => "[]".into(),
        };
    }
    "[]".into()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn get_orders() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        return match safe_local_storage() {
            Ok(s) => s
                .get_item(KEY_ORDERS)
                .ok()
                .flatten()
                .unwrap_or_else(|| "[]".into()),
            Err(_) => "[]".into(),
        };
    }
    "[]".into()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn import_deliveries(json: &str) -> Result<(), String> {
    let deliveries: Vec<models::Delivery> = parse_json_vec(json)?;
    #[cfg(target_arch = "wasm32")]
    {
        safe_local_storage()?
            .set_item(KEY_DELIVERIES, &serde_json::to_string(&deliveries).unwrap())
            .map_err(|_| "failed to persist deliveries".to_string())?;
    }
    Ok(())
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn export_deliveries() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        return match safe_local_storage() {
            Ok(s) => s
                .get_item(KEY_DELIVERIES)
                .ok()
                .flatten()
                .unwrap_or_else(|| "[]".into()),
            Err(_) => "[]".into(),
        };
    }
    "[]".into()
}

// ---------- Business logic helpers (pure Rust) ----------
fn now_iso_utc() -> String {
    // We avoid timezone libs for native tests; use RFC3339 via chrono if added later.
    // Here we construct a simple ISO8601 using js_sys when in wasm, else fallback to UTC from std.
    #[cfg(target_arch = "wasm32")]
    {
        use js_sys::Date;
        return Date::new_0().to_iso_string().into();
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Minimal UTC timestamp using std time; not including nanos for simplicity
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        // Format as seconds since epoch for determinism in tests; callers don't assert exact format.
        format!("{}s", now.as_secs())
    }
}

fn load_orders_from_storage() -> Vec<models::Order> {
    #[cfg(target_arch = "wasm32")]
    {
        if let Ok(storage) = safe_local_storage() {
            if let Ok(Some(s)) = storage.get_item(KEY_ORDERS) {
                serde_json::from_str(&s).unwrap_or_default()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        Vec::new()
    }
}

fn load_deliveries_from_storage() -> Vec<models::Delivery> {
    #[cfg(target_arch = "wasm32")]
    {
        if let Ok(storage) = safe_local_storage() {
            if let Ok(Some(s)) = storage.get_item(KEY_DELIVERIES) {
                serde_json::from_str(&s).unwrap_or_default()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        Vec::new()
    }
}

#[cfg(target_arch = "wasm32")]
fn save_deliveries_to_storage(deliveries: &[models::Delivery]) -> Result<(), String> {
    safe_local_storage()?
        .set_item(KEY_DELIVERIES, &serde_json::to_string(deliveries).unwrap())
        .map_err(|_| "failed to persist deliveries".to_string())
}

// Transition rules helpers (pure): mutate deliveries vector and return message or error
fn find_order<'a>(orders: &'a [models::Order], number: u32) -> Option<&'a models::Order> {
    orders.iter().find(|o| o.number == number)
}

fn user_id() -> Option<u32> {
    Some(1)
}

fn has_active_or_stored(deliveries: &[models::Delivery], order_number: u32) -> bool {
    deliveries.iter().any(|d| {
        d.order_number == order_number
            && matches!(
                d.status,
                models::DeliveryStatus::InProgress | models::DeliveryStatus::STORED
            )
    })
}

fn get_mut_delivery<'a>(
    deliveries: &'a mut [models::Delivery],
    order_number: u32,
) -> Option<&'a mut models::Delivery> {
    deliveries
        .iter_mut()
        .find(|d| d.order_number == order_number)
}

fn get_active_delivery_mut<'a>(
    deliveries: &'a mut [models::Delivery],
    order_number: u32,
) -> Option<&'a mut models::Delivery> {
    deliveries.iter_mut().find(|d| {
        d.order_number == order_number
            && matches!(
                d.status,
                models::DeliveryStatus::InProgress | models::DeliveryStatus::STORED
            )
    })
}

fn make_new_delivery(order_number: u32, status: models::DeliveryStatus) -> models::Delivery {
    models::Delivery {
        id: order_number, // simple id for PoC; in real app use UUID/sequence
        order_number,
        status,
        location_id: None,
        started_at: Some(now_iso_utc()),
        ended_at: None,
        comment: None,
        user_id: user_id(),
    }
}

fn logic_take_order(
    orders: &[models::Order],
    deliveries: &mut Vec<models::Delivery>,
    order_number: u32,
) -> Result<String, String> {
    let _order = find_order(orders, order_number)
        .ok_or_else(|| format!("order {} not found", order_number))?;
    if has_active_or_stored(deliveries, order_number) {
        return Err("delivery already active or stored for this order".into());
    }
    deliveries.push(make_new_delivery(
        order_number,
        models::DeliveryStatus::InProgress,
    ));
    Ok(format!("order {} taken", order_number))
}

fn logic_store_delivery(
    deliveries: &mut Vec<models::Delivery>,
    order_number: u32,
    location_id: u32,
    comment: Option<String>,
) -> Result<String, String> {
    let d = get_active_delivery_mut(deliveries, order_number)
        .ok_or_else(|| "delivery not found".to_string())?;
    match d.status {
        models::DeliveryStatus::InProgress => {
            d.status = models::DeliveryStatus::STORED;
            d.location_id = Some(location_id);
            if let Some(c) = comment {
                d.comment = Some(c);
            }
            Ok("stored".into())
        }
        _ => Err("can only store an in-progress delivery".into()),
    }
}

fn logic_continue_delivery(
    deliveries: &mut Vec<models::Delivery>,
    order_number: u32,
    comment: Option<String>,
) -> Result<String, String> {
    let d = get_active_delivery_mut(deliveries, order_number)
        .ok_or_else(|| "delivery not found".to_string())?;
    match d.status {
        models::DeliveryStatus::STORED => {
            d.status = models::DeliveryStatus::InProgress;
            d.location_id = None;
            if let Some(c) = comment {
                d.comment = Some(c);
            }
            Ok("continued".into())
        }
        _ => Err("can only continue a stored delivery".into()),
    }
}

fn logic_make_delivery(
    orders: &[models::Order],
    deliveries: &mut Vec<models::Delivery>,
    order_number: u32,
) -> Result<String, String> {
    let order = find_order(orders, order_number).ok_or_else(|| "order not found".to_string())?;
    // Prefer completing the active delivery if present; otherwise create a new one
    let active = get_active_delivery_mut(deliveries, order_number).is_some();
    if !active {
        deliveries.push(make_new_delivery(
            order_number,
            models::DeliveryStatus::InProgress,
        ));
    }
    let d = get_active_delivery_mut(deliveries, order_number).expect("delivery must exist");
    d.status = models::DeliveryStatus::COMPLETE;
    d.ended_at = Some(now_iso_utc());
    d.location_id = Some(order.client_id);
    Ok("completed".into())
}

fn logic_fail_delivery(
    orders: &[models::Order],
    deliveries: &mut Vec<models::Delivery>,
    order_number: u32,
    comment: Option<String>,
) -> Result<String, String> {
    let order = find_order(orders, order_number).ok_or_else(|| "order not found".to_string())?;
    let d = get_active_delivery_mut(deliveries, order_number)
        .ok_or_else(|| "delivery not found".to_string())?;
    d.status = models::DeliveryStatus::FAILED;
    d.ended_at = Some(now_iso_utc());
    d.location_id = Some(order.destination_id);
    if let Some(c) = comment {
        d.comment = Some(c);
    }
    Ok("failed".into())
}

fn logic_lose_delivery(
    deliveries: &mut Vec<models::Delivery>,
    order_number: u32,
    comment: Option<String>,
) -> Result<String, String> {
    let d = get_active_delivery_mut(deliveries, order_number)
        .ok_or_else(|| "delivery not found".to_string())?;
    d.status = models::DeliveryStatus::LOST;
    d.ended_at = Some(now_iso_utc());
    if let Some(c) = comment {
        d.comment = Some(c);
    }
    Ok("lost".into())
}

fn logic_bulk_accept(
    orders: &[models::Order],
    deliveries: &mut Vec<models::Delivery>,
    numbers: &[u32],
) -> Result<String, String> {
    let mut created = 0usize;
    for &n in numbers {
        if find_order(orders, n).is_none() {
            continue;
        }
        if !has_active_or_stored(deliveries, n) {
            deliveries.push(make_new_delivery(n, models::DeliveryStatus::InProgress));
            created += 1;
        }
    }
    Ok(format!("accepted {}", created))
}

fn logic_bulk_complete(
    orders: &[models::Order],
    deliveries: &mut Vec<models::Delivery>,
    numbers: &[u32],
) -> Result<String, String> {
    let mut completed = 0usize;
    for &n in numbers {
        if find_order(orders, n).is_none() {
            continue;
        }
        if get_mut_delivery(deliveries, n).is_none() {
            deliveries.push(make_new_delivery(n, models::DeliveryStatus::InProgress));
        }
        let _ = logic_make_delivery(orders, deliveries, n)?;
        completed += 1;
    }
    Ok(format!("completed {}", completed))
}

// ---------- Validation helpers ----------
fn validate_nonzero(name: &str, value: u32) -> Result<(), String> {
    if value == 0 {
        Err(format!("{} must be > 0", name))
    } else {
        Ok(())
    }
}

fn validate_comment(comment: &Option<String>) -> Result<(), String> {
    if let Some(c) = comment {
        if c.len() > 500 {
            return Err("comment too long (max 500 chars)".into());
        }
    }
    Ok(())
}

// ---------- WASM exposed business logic ----------
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn initialize() {
    // No-op for now; data import functions seed state.
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn take_order(order_number: u32) -> Result<String, String> {
    validate_nonzero("order_number", order_number)?;
    let orders = load_orders_from_storage();
    let mut deliveries = load_deliveries_from_storage();
    let res = logic_take_order(&orders, &mut deliveries, order_number);
    #[cfg(target_arch = "wasm32")]
    if res.is_ok() {
        save_deliveries_to_storage(&deliveries)?;
    }
    res
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn store_delivery(
    order_number: u32,
    location_id: u32,
    comment: Option<String>,
) -> Result<String, String> {
    validate_nonzero("order_number", order_number)?;
    validate_nonzero("location_id", location_id)?;
    validate_comment(&comment)?;
    let mut deliveries = load_deliveries_from_storage();
    let res = logic_store_delivery(&mut deliveries, order_number, location_id, comment);
    #[cfg(target_arch = "wasm32")]
    if res.is_ok() {
        save_deliveries_to_storage(&deliveries)?;
    }
    res
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn continue_delivery(order_number: u32, comment: Option<String>) -> Result<String, String> {
    validate_nonzero("order_number", order_number)?;
    validate_comment(&comment)?;
    let mut deliveries = load_deliveries_from_storage();
    let res = logic_continue_delivery(&mut deliveries, order_number, comment);
    #[cfg(target_arch = "wasm32")]
    if res.is_ok() {
        save_deliveries_to_storage(&deliveries)?;
    }
    res
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn make_delivery(order_number: u32) -> Result<String, String> {
    validate_nonzero("order_number", order_number)?;
    let orders = load_orders_from_storage();
    let mut deliveries = load_deliveries_from_storage();
    let res = logic_make_delivery(&orders, &mut deliveries, order_number);
    #[cfg(target_arch = "wasm32")]
    if res.is_ok() {
        save_deliveries_to_storage(&deliveries)?;
    }
    res
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn fail_delivery(order_number: u32, comment: Option<String>) -> Result<String, String> {
    validate_nonzero("order_number", order_number)?;
    validate_comment(&comment)?;
    let orders = load_orders_from_storage();
    let mut deliveries = load_deliveries_from_storage();
    let res = logic_fail_delivery(&orders, &mut deliveries, order_number, comment);
    #[cfg(target_arch = "wasm32")]
    if res.is_ok() {
        save_deliveries_to_storage(&deliveries)?;
    }
    res
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn lose_delivery(order_number: u32, comment: Option<String>) -> Result<String, String> {
    validate_nonzero("order_number", order_number)?;
    validate_comment(&comment)?;
    let mut deliveries = load_deliveries_from_storage();
    let res = logic_lose_delivery(&mut deliveries, order_number, comment);
    #[cfg(target_arch = "wasm32")]
    if res.is_ok() {
        save_deliveries_to_storage(&deliveries)?;
    }
    res
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn bulk_accept(order_numbers: Vec<u32>) -> Result<String, String> {
    if order_numbers.iter().any(|&n| n == 0) {
        return Err("order_numbers contain invalid 0 value".into());
    }
    let orders = load_orders_from_storage();
    let mut deliveries = load_deliveries_from_storage();
    let res = logic_bulk_accept(&orders, &mut deliveries, &order_numbers);
    #[cfg(target_arch = "wasm32")]
    if res.is_ok() {
        save_deliveries_to_storage(&deliveries)?;
    }
    res
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn bulk_complete(order_numbers: Vec<u32>) -> Result<String, String> {
    if order_numbers.iter().any(|&n| n == 0) {
        return Err("order_numbers contain invalid 0 value".into());
    }
    let orders = load_orders_from_storage();
    let mut deliveries = load_deliveries_from_storage();
    let res = logic_bulk_complete(&orders, &mut deliveries, &order_numbers);
    #[cfg(target_arch = "wasm32")]
    if res.is_ok() {
        save_deliveries_to_storage(&deliveries)?;
    }
    res
}

#[cfg(target_arch = "wasm32")]
fn local_storage() -> Storage {
    // Kept for backward-compat in internal calls; may panic if storage unavailable.
    window().unwrap().local_storage().unwrap().unwrap()
}

#[cfg(target_arch = "wasm32")]
fn safe_local_storage() -> Result<Storage, String> {
    let win = window().ok_or_else(|| "window unavailable".to_string())?;
    let storage = win
        .local_storage()
        .map_err(|_| "localStorage unavailable".to_string())?
        .ok_or_else(|| "localStorage not accessible".to_string())?;
    Ok(storage)
}

// ---------- Query and Summary services ----------
#[cfg(target_arch = "wasm32")]
fn load_locations_from_storage() -> Vec<models::Location> {
    if let Ok(storage) = safe_local_storage() {
        if let Ok(Some(s)) = storage.get_item(KEY_LOCATIONS) {
            serde_json::from_str(&s).unwrap_or_default()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    }
}

#[cfg(target_arch = "wasm32")]
fn load_districts_from_storage() -> Vec<models::District> {
    if let Ok(storage) = safe_local_storage() {
        if let Ok(Some(s)) = storage.get_item(KEY_DISTRICTS) {
            serde_json::from_str(&s).unwrap_or_default()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn query_orders(
    filter_json: Option<String>,
    page: u32,
    per_page: u32,
    sort_key: Option<String>,
    sort_dir: Option<String>,
    search: Option<String>,
) -> String {
    use crate::services::{
        OrdersFilter, QueryResult, SortDir, SortKey, map_orders_to_list_items, paginate,
        search_orders, sort_orders,
    };

    // Load data only in wasm builds; native builds return empty result to keep cfg clean
    #[cfg(target_arch = "wasm32")]
    {
        let orders = load_orders_from_storage();
        let deliveries = load_deliveries_from_storage();
        let locations = load_locations_from_storage();

        let mut filter: OrdersFilter = match filter_json.as_ref() {
            Some(s) if !s.is_empty() => serde_json::from_str(s).unwrap_or_default(),
            _ => OrdersFilter::default(),
        };
        // In case we later add defaults, ensure deserialization didn't leave filter uninitialized
        let filtered_refs = services::filter_orders(&orders, &deliveries, &locations, &filter);
        let filtered_orders: Vec<models::Order> = filtered_refs.into_iter().cloned().collect();
        let mut list = map_orders_to_list_items(&filtered_orders, &deliveries);

        // Sorting
        let key = match sort_key.as_deref() {
            Some("name") => SortKey::Name,
            Some("weight") => SortKey::Weight,
            Some("max_likes") => SortKey::MaxLikes,
            _ => SortKey::Number,
        };
        let dir = match sort_dir.as_deref() {
            Some("desc") => SortDir::Desc,
            _ => SortDir::Asc,
        };
        sort_orders(&mut list, key, dir);

        // Search
        let list = if let Some(q) = search {
            search_orders(&list, &q)
        } else {
            list
        };

        // Pagination
        let (total, items) = paginate(&list, page.max(1), per_page.max(1));
        let result = QueryResult { total, items };
        return serde_json::to_string(&result)
            .unwrap_or_else(|_| "{\"total\":0,\"items\":[]}".into());
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        return "{\"total\":0,\"items\":[]}".into();
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn get_dashboard_summary() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        let orders = load_orders_from_storage();
        let deliveries = load_deliveries_from_storage();
        let locations = load_locations_from_storage();
        let districts = load_districts_from_storage();
        let summary =
            services::compute_dashboard_summary(&orders, &deliveries, &locations, &districts);
        return serde_json::to_string(&summary).unwrap_or_else(|_| "{}".into());
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        return "{}".into();
    }
}
