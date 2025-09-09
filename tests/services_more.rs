use death_stranding_poc::models::{Delivery, DeliveryStatus, Location, Order};
use death_stranding_poc::services::{
    DeliveryStatusFilter, OrderListItem, OrdersFilter, SortDir, SortKey,
    current_user_status_and_completed, filter_orders, paginate, search_orders, sort_orders,
};

fn o(
    number: u32,
    name: &str,
    client_id: u32,
    destination_id: u32,
    cat: u32,
    max_likes: f32,
    weight: f32,
) -> Order {
    Order {
        number,
        name: name.into(),
        client_id,
        destination_id,
        delivery_category_id: cat,
        max_likes,
        weight,
    }
}

fn l(id: u32, district_id: u32) -> Location {
    Location {
        id,
        name: format!("L{}", id),
        district_id,
        is_physical: true,
    }
}

fn d(id: u32, order_number: u32, status: DeliveryStatus, user_id: Option<u32>) -> Delivery {
    Delivery {
        id,
        order_number,
        status,
        location_id: None,
        started_at: None,
        ended_at: None,
        comment: None,
        user_id,
    }
}

#[test]
fn delivery_status_filters_cover_all_variants() {
    let orders = vec![
        o(1, "A", 10, 20, 1, 0.0, 1.0),
        o(2, "B", 11, 21, 1, 0.0, 1.0),
    ];
    let locations = vec![l(10, 1), l(11, 1), l(20, 2), l(21, 2)];
    let deliveries = vec![
        d(1, 1, DeliveryStatus::InProgress, Some(1)),
        d(2, 2, DeliveryStatus::STORED, Some(1)),
        d(3, 1, DeliveryStatus::COMPLETE, Some(1)),
        d(4, 2, DeliveryStatus::FAILED, Some(1)),
        d(5, 1, DeliveryStatus::LOST, Some(1)),
    ];

    let mut f = OrdersFilter::default();
    f.delivery_status = Some(DeliveryStatusFilter::Any);
    assert_eq!(filter_orders(&orders, &deliveries, &locations, &f).len(), 2);

    f.delivery_status = Some(DeliveryStatusFilter::None);
    assert_eq!(filter_orders(&orders, &deliveries, &locations, &f).len(), 0);

    f.delivery_status = Some(DeliveryStatusFilter::InProgress);
    let nums: Vec<u32> = filter_orders(&orders, &deliveries, &locations, &f)
        .into_iter()
        .map(|o| o.number)
        .collect();
    assert_eq!(nums, vec![1]);

    f.delivery_status = Some(DeliveryStatusFilter::Stored);
    let nums: Vec<u32> = filter_orders(&orders, &deliveries, &locations, &f)
        .into_iter()
        .map(|o| o.number)
        .collect();
    assert_eq!(nums, vec![2]);

    f.delivery_status = Some(DeliveryStatusFilter::Complete);
    let nums: Vec<u32> = filter_orders(&orders, &deliveries, &locations, &f)
        .into_iter()
        .map(|o| o.number)
        .collect();
    assert_eq!(nums, vec![1]);

    f.delivery_status = Some(DeliveryStatusFilter::Failed);
    let nums: Vec<u32> = filter_orders(&orders, &deliveries, &locations, &f)
        .into_iter()
        .map(|o| o.number)
        .collect();
    assert_eq!(nums, vec![2]);

    f.delivery_status = Some(DeliveryStatusFilter::Lost);
    let nums: Vec<u32> = filter_orders(&orders, &deliveries, &locations, &f)
        .into_iter()
        .map(|o| o.number)
        .collect();
    assert_eq!(nums, vec![1]);
}

#[test]
fn district_filter_checks_client_or_destination() {
    let orders = vec![
        o(1, "A", 10, 20, 1, 0.0, 1.0),
        o(2, "B", 12, 11, 1, 0.0, 1.0),
        o(3, "C", 30, 31, 1, 0.0, 1.0),
    ];
    let locations = vec![l(10, 1), l(11, 1), l(12, 2), l(20, 3), l(30, 4), l(31, 5)];
    let deliveries: Vec<Delivery> = vec![];
    let mut f = OrdersFilter::default();
    f.district_id = Some(1);
    let nums: Vec<u32> = filter_orders(&orders, &deliveries, &locations, &f)
        .into_iter()
        .map(|o| o.number)
        .collect();
    // Order 1 has client in district 1; Order 2 has destination in district 1; Order 3 neither
    assert_eq!(nums, vec![1, 2]);
}

#[test]
fn completion_true_and_false() {
    let orders = vec![
        o(1, "A", 10, 20, 1, 0.0, 1.0),
        o(2, "B", 11, 21, 1, 0.0, 1.0),
        o(3, "C", 12, 22, 1, 0.0, 1.0),
    ];
    let locations = vec![l(10, 1), l(11, 1), l(12, 1), l(20, 2), l(21, 2), l(22, 2)];
    let deliveries = vec![
        d(1, 1, DeliveryStatus::COMPLETE, Some(1)),
        d(2, 2, DeliveryStatus::FAILED, Some(1)),
    ];

    let mut f = OrdersFilter::default();
    f.completion = Some(true);
    let completed_nums: Vec<u32> = filter_orders(&orders, &deliveries, &locations, &f)
        .into_iter()
        .map(|o| o.number)
        .collect();
    assert_eq!(completed_nums, vec![1]);

    f.completion = Some(false);
    let not_completed_nums: Vec<u32> = filter_orders(&orders, &deliveries, &locations, &f)
        .into_iter()
        .map(|o| o.number)
        .collect();
    // 2 has delivery but not complete, 3 has no delivery
    assert_eq!(not_completed_nums, vec![2, 3]);
}

#[test]
fn user_scoping_for_current_user_status_and_completed() {
    let deliveries = vec![
        d(1, 1, DeliveryStatus::COMPLETE, Some(2)), // other user
        d(2, 1, DeliveryStatus::FAILED, Some(1)),   // current user
        d(3, 2, DeliveryStatus::InProgress, None),  // no user_id -> treated as current
    ];
    let (status1, completed1) = current_user_status_and_completed(1, &deliveries);
    assert_eq!(status1, Some(DeliveryStatus::FAILED));
    assert_eq!(completed1, false);

    let (status2, completed2) = current_user_status_and_completed(2, &deliveries);
    assert_eq!(status2, Some(DeliveryStatus::InProgress));
    assert_eq!(completed2, false);
}

#[test]
fn sorting_stability_and_ties() {
    let mut items = vec![
        OrderListItem {
            number: 1,
            name: "alpha".into(),
            client_id: 0,
            destination_id: 0,
            delivery_category_id: 0,
            max_likes: 2.0,
            weight: 1.0,
            delivery_status: None,
            is_completed: false,
        },
        OrderListItem {
            number: 2,
            name: "Alpha".into(),
            client_id: 0,
            destination_id: 0,
            delivery_category_id: 0,
            max_likes: 2.0,
            weight: 1.0,
            delivery_status: None,
            is_completed: false,
        },
        OrderListItem {
            number: 3,
            name: "beta".into(),
            client_id: 0,
            destination_id: 0,
            delivery_category_id: 0,
            max_likes: 2.0,
            weight: 1.0,
            delivery_status: None,
            is_completed: false,
        },
    ];
    sort_orders(&mut items, SortKey::Name, SortDir::Asc);
    let names: Vec<String> = items.iter().map(|i| i.name.clone()).collect();
    assert_eq!(names, vec!["alpha", "Alpha", "beta"]);

    sort_orders(&mut items, SortKey::Number, SortDir::Desc);
    let numbers: Vec<u32> = items.iter().map(|i| i.number).collect();
    assert_eq!(numbers, vec![3, 2, 1]);
}

#[test]
fn paginate_edge_cases() {
    let items: Vec<u32> = (1..=5).collect();
    // per_page=0 returns empty but keeps total
    let (total, page) = paginate(&items, 1, 0);
    assert_eq!(total, 5);
    assert!(page.is_empty());

    // page=0 clamps to 0 via saturating_sub, starts at 0
    let (_total, page) = paginate(&items, 0, 2);
    assert_eq!(page, vec![1, 2]);

    // page beyond range returns empty but total preserved
    let (total2, page2) = paginate(&items, 10, 2);
    assert_eq!(total2, 5);
    assert!(page2.is_empty());
}

#[test]
fn search_matches_number_and_empty_query_returns_all() {
    let items = vec![
        OrderListItem {
            number: 42,
            name: "Package".into(),
            client_id: 0,
            destination_id: 0,
            delivery_category_id: 0,
            max_likes: 0.0,
            weight: 0.0,
            delivery_status: None,
            is_completed: false,
        },
        OrderListItem {
            number: 7,
            name: "Cargo".into(),
            client_id: 0,
            destination_id: 0,
            delivery_category_id: 0,
            max_likes: 0.0,
            weight: 0.0,
            delivery_status: None,
            is_completed: false,
        },
    ];
    let s = search_orders(&items, "42");
    assert_eq!(s.len(), 1);
    assert_eq!(s[0].number, 42);

    let all = search_orders(&items, "   ");
    assert_eq!(all.len(), items.len());
}
