use crate::models::{Delivery, DeliveryStatus, District, Location, Order, Region};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct OrdersFilter {
    pub district_id: Option<u32>,
    pub client_id: Option<u32>,
    pub destination_id: Option<u32>,
    pub delivery_category_id: Option<u32>,
    /// Filter by current user's delivery status for an order, if any
    pub delivery_status: Option<DeliveryStatusFilter>,
    /// Completion filter: true => has COMPLETE for current user; false => does not have COMPLETE
    pub completion: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DeliveryStatusFilter {
    InProgress,
    Stored,
    Complete,
    Failed,
    Lost,
    /// Any existing delivery regardless of status
    Any,
    /// No delivery for current user
    None,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SortKey {
    Number,
    Name,
    Weight,
    MaxLikes,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SortDir {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrderListItem {
    pub number: u32,
    pub name: String,
    pub client_id: u32,
    pub destination_id: u32,
    pub delivery_category_id: u32,
    pub max_likes: f32,
    pub weight: f32,
    pub delivery_status: Option<DeliveryStatus>,
    pub is_completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResult<T> {
    pub total: usize,
    pub items: Vec<T>,
}

pub fn map_orders_to_list_items(orders: &[Order], deliveries: &[Delivery]) -> Vec<OrderListItem> {
    orders
        .iter()
        .map(|o| {
            let (status, completed) = current_user_status_and_completed(o.number, deliveries);
            OrderListItem {
                number: o.number,
                name: o.name.clone(),
                client_id: o.client_id,
                destination_id: o.destination_id,
                delivery_category_id: o.delivery_category_id,
                max_likes: o.max_likes,
                weight: o.weight,
                delivery_status: status,
                is_completed: completed,
            }
        })
        .collect()
}

pub fn current_user_status_and_completed(
    order_number: u32,
    deliveries: &[Delivery],
) -> (Option<DeliveryStatus>, bool) {
    let mut has_complete = bool::default();
    let mut has_failed = bool::default();
    let mut has_lost = bool::default();
    let mut active_status: Option<DeliveryStatus> = None;

    for d in deliveries.iter().filter(|d| d.order_number == order_number) {
        if let Some(uid) = d.user_id {
            if uid != 1 {
                continue; // only current PoC user
            }
        }
        match d.status {
            DeliveryStatus::InProgress | DeliveryStatus::STORED => {
                // prefer showing active if any
                active_status = Some(d.status.clone());
            }
            DeliveryStatus::COMPLETE => has_complete = true,
            DeliveryStatus::FAILED => has_failed = true,
            DeliveryStatus::LOST => has_lost = true,
        }
    }

    if let Some(s) = active_status {
        return (Some(s), has_complete);
    }
    if has_complete {
        return (Some(DeliveryStatus::COMPLETE), true);
    }
    if has_failed {
        return (Some(DeliveryStatus::FAILED), false);
    }
    if has_lost {
        return (Some(DeliveryStatus::LOST), false);
    }
    (None, false)
}

pub fn filter_orders<'a>(
    orders: &'a [Order],
    deliveries: &'a [Delivery],
    locations: &'a [Location],
    f: &OrdersFilter,
) -> Vec<&'a Order> {
    let mut result: Vec<&'a Order> = orders.iter().collect();

    if let Some(district_id) = f.district_id {
        // Build set of location ids in that district
        let ids: std::collections::HashSet<u32> = locations
            .iter()
            .filter(|l| l.district_id == district_id)
            .map(|l| l.id)
            .collect();
        result.retain(|o| ids.contains(&o.client_id) || ids.contains(&o.destination_id));
    }

    if let Some(cid) = f.client_id {
        result.retain(|o| o.client_id == cid);
    }

    if let Some(did) = f.destination_id {
        result.retain(|o| o.destination_id == did);
    }

    if let Some(cat) = f.delivery_category_id {
        result.retain(|o| o.delivery_category_id == cat);
    }

    if let Some(ds) = &f.delivery_status {
        match ds {
            DeliveryStatusFilter::Any => {
                result.retain(|o| deliveries.iter().any(|d| d.order_number == o.number));
            }
            DeliveryStatusFilter::None => {
                result.retain(|o| deliveries.iter().all(|d| d.order_number != o.number));
            }
            DeliveryStatusFilter::InProgress => {
                result.retain(|o| {
                    deliveries.iter().any(|d| {
                        d.order_number == o.number && matches!(d.status, DeliveryStatus::InProgress)
                    })
                });
            }
            DeliveryStatusFilter::Stored => {
                result.retain(|o| {
                    deliveries.iter().any(|d| {
                        d.order_number == o.number && matches!(d.status, DeliveryStatus::STORED)
                    })
                });
            }
            DeliveryStatusFilter::Complete => {
                result.retain(|o| {
                    deliveries.iter().any(|d| {
                        d.order_number == o.number && matches!(d.status, DeliveryStatus::COMPLETE)
                    })
                });
            }
            DeliveryStatusFilter::Failed => {
                result.retain(|o| {
                    deliveries.iter().any(|d| {
                        d.order_number == o.number && matches!(d.status, DeliveryStatus::FAILED)
                    })
                });
            }
            DeliveryStatusFilter::Lost => {
                result.retain(|o| {
                    deliveries.iter().any(|d| {
                        d.order_number == o.number && matches!(d.status, DeliveryStatus::LOST)
                    })
                });
            }
        }
    }

    if let Some(completion) = f.completion {
        if completion {
            result.retain(|o| {
                deliveries.iter().any(|d| {
                    d.order_number == o.number && matches!(d.status, DeliveryStatus::COMPLETE)
                })
            });
        } else {
            result.retain(|o| {
                deliveries.iter().all(|d| {
                    !(d.order_number == o.number && matches!(d.status, DeliveryStatus::COMPLETE))
                })
            });
        }
    }

    result
}

pub fn sort_orders(items: &mut [OrderListItem], key: SortKey, dir: SortDir) {
    match key {
        SortKey::Number => items.sort_by_key(|i| i.number),
        SortKey::Name => items.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
        SortKey::Weight => items.sort_by(|a, b| {
            a.weight
                .partial_cmp(&b.weight)
                .unwrap_or(std::cmp::Ordering::Equal)
        }),
        SortKey::MaxLikes => items.sort_by(|a, b| {
            a.max_likes
                .partial_cmp(&b.max_likes)
                .unwrap_or(std::cmp::Ordering::Equal)
        }),
    }
    if let SortDir::Desc = dir {
        items.reverse();
    }
}

pub fn search_orders(items: &[OrderListItem], q: &str) -> Vec<OrderListItem> {
    if q.trim().is_empty() {
        return items.to_vec();
    }
    let needle = q.to_ascii_lowercase();
    items
        .iter()
        .filter(|i| {
            i.name.to_ascii_lowercase().contains(&needle) || i.number.to_string().contains(&needle)
        })
        .cloned()
        .collect()
}

pub fn paginate<T: Clone>(items: &[T], page: u32, per_page: u32) -> (usize, Vec<T>) {
    let total = items.len();
    if per_page == 0 {
        return (total, Vec::new());
    }
    let start = (page.saturating_sub(1) as usize) * per_page as usize;
    let end = std::cmp::min(start + per_page as usize, total);
    if start >= total {
        return (total, Vec::new());
    }
    (total, items[start..end].to_vec())
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DashboardSummary {
    pub central_total: u32,
    pub central_ae: u32,
    pub central_fm: u32,
    pub central_nw: u32,
    pub east: u32,
    pub west: u32,
}

/// Compute summary of COMPLETED deliveries grouped by destination region and Central subgroups by destination location name.
/// Counting logic:
/// - A delivery counts if its status == COMPLETE.
/// - It is attributed to the order's destination Location, then to that Location's District.region.
/// - For Central region, further bucket by destination Location name first letter: A–E, F–M, N–W (case-insensitive).
pub fn compute_dashboard_summary(
    orders: &[Order],
    deliveries: &[Delivery],
    locations: &[Location],
    districts: &[District],
) -> DashboardSummary {
    use std::collections::HashMap;

    let loc_by_id: HashMap<u32, &Location> = locations.iter().map(|l| (l.id, l)).collect();
    let dist_by_id: HashMap<u32, &District> = districts.iter().map(|d| (d.id, d)).collect();
    let order_by_num: HashMap<u32, &Order> = orders.iter().map(|o| (o.number, o)).collect();

    let mut out = DashboardSummary::default();

    for d in deliveries.iter() {
        if d.status != DeliveryStatus::COMPLETE {
            continue;
        }
        let Some(order) = order_by_num.get(&d.order_number) else {
            continue;
        };
        let Some(loc) = loc_by_id.get(&order.destination_id) else {
            continue;
        };
        let Some(dist) = dist_by_id.get(&loc.district_id) else {
            continue;
        };
        match dist.region {
            Region::East => {
                out.east += 1;
            }
            Region::West => {
                out.west += 1;
            }
            Region::Central => {
                out.central_total += 1;
                // Group by first letter of location name
                let first = loc
                    .name
                    .chars()
                    .find(|c| c.is_alphabetic())
                    .map(|c| c.to_ascii_uppercase());
                if let Some(ch) = first {
                    if ('A'..='E').contains(&ch) {
                        out.central_ae += 1;
                    } else if ('F'..='M').contains(&ch) {
                        out.central_fm += 1;
                    } else if ('N'..='W').contains(&ch) {
                        out.central_nw += 1;
                    }
                }
            }
        }
    }

    out
}
