use death_stranding_poc::models::{Delivery, DeliveryStatus, Location, Order};
use death_stranding_poc::services::{
    OrdersFilter, SortDir, SortKey, filter_orders, map_orders_to_list_items, paginate,
    search_orders, sort_orders,
};

fn sample_orders() -> Vec<Order> {
    vec![
        Order {
            number: 1,
            name: "A".into(),
            client_id: 10,
            destination_id: 20,
            delivery_category_id: 100,
            max_likes: 5.0,
            weight: 2.0,
        },
        Order {
            number: 2,
            name: "B".into(),
            client_id: 11,
            destination_id: 21,
            delivery_category_id: 101,
            max_likes: 3.0,
            weight: 5.0,
        },
        Order {
            number: 3,
            name: "Cargo".into(),
            client_id: 12,
            destination_id: 20,
            delivery_category_id: 100,
            max_likes: 9.0,
            weight: 1.0,
        },
    ]
}

fn sample_locations() -> Vec<Location> {
    vec![
        Location {
            id: 10,
            name: "L10".into(),
            district_id: 1,
            is_physical: true,
        },
        Location {
            id: 11,
            name: "L11".into(),
            district_id: 1,
            is_physical: true,
        },
        Location {
            id: 12,
            name: "L12".into(),
            district_id: 2,
            is_physical: true,
        },
        Location {
            id: 20,
            name: "L20".into(),
            district_id: 3,
            is_physical: true,
        },
        Location {
            id: 21,
            name: "L21".into(),
            district_id: 2,
            is_physical: true,
        },
    ]
}

fn sample_deliveries() -> Vec<Delivery> {
    vec![
        Delivery {
            id: 1,
            order_number: 1,
            status: DeliveryStatus::InProgress,
            location_id: None,
            started_at: None,
            ended_at: None,
            comment: None,
            user_id: Some(1),
        },
        Delivery {
            id: 2,
            order_number: 2,
            status: DeliveryStatus::COMPLETE,
            location_id: None,
            started_at: None,
            ended_at: None,
            comment: None,
            user_id: Some(1),
        },
    ]
}

#[test]
fn filter_by_district() {
    let orders = sample_orders();
    let deliveries = sample_deliveries();
    let locations = sample_locations();
    let mut f = OrdersFilter::default();
    f.district_id = Some(1);
    let filtered = filter_orders(&orders, &deliveries, &locations, &f);
    let nums: Vec<u32> = filtered.into_iter().map(|o| o.number).collect();
    assert_eq!(nums, vec![1, 2]); // orders with client in district 1
}

#[test]
fn filter_by_category_and_completion() {
    let orders = sample_orders();
    let deliveries = sample_deliveries();
    let locations = sample_locations();
    let mut f = OrdersFilter::default();
    f.delivery_category_id = Some(100);
    f.completion = Some(false);
    let filtered = filter_orders(&orders, &deliveries, &locations, &f);
    let nums: Vec<u32> = filtered.into_iter().map(|o| o.number).collect();
    assert_eq!(nums, vec![1, 3]);
    // of these, only order 1 is in deliveries (in progress), order 3 has none
}

#[test]
fn sort_and_search_and_paginate() {
    let list_items = map_orders_to_list_items(&sample_orders(), &sample_deliveries());
    let mut items = list_items.clone();
    sort_orders(&mut items, SortKey::Weight, SortDir::Asc);
    let weights: Vec<f32> = items.iter().map(|i| i.weight).collect();
    assert_eq!(weights, vec![1.0, 2.0, 5.0]);

    let searched = search_orders(&items, "car");
    assert_eq!(searched.len(), 1);
    assert_eq!(searched[0].name, "Cargo");

    let (total, page) = paginate(&items, 2, 2);
    assert_eq!(total, 3);
    assert_eq!(page.len(), 1);
    assert_eq!(page[0].number, 2);
}
