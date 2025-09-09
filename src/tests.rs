use super::*;

fn order(n: u32, client: u32, dest: u32) -> models::Order {
    models::Order {
        number: n,
        name: format!("Order {}", n),
        client_id: client,
        destination_id: dest,
        delivery_category_id: 1,
        max_likes: 0.0,
        weight: 0.0,
    }
}

#[test]
fn status_transitions_happy_paths() {
    let orders = vec![order(1, 100, 200)];
    let mut deliveries: Vec<models::Delivery> = vec![];

    // take
    let msg = logic_take_order(&orders, &mut deliveries, 1).unwrap();
    assert_eq!(msg, "order 1 taken");
    assert_eq!(deliveries.len(), 1);
    assert_eq!(deliveries[0].status, models::DeliveryStatus::InProgress);
    assert!(deliveries[0].started_at.is_some());

    // store
    let msg = logic_store_delivery(&mut deliveries, 1, 500, Some("pausing".into())).unwrap();
    assert_eq!(msg, "stored");
    assert_eq!(deliveries[0].status, models::DeliveryStatus::STORED);
    assert_eq!(deliveries[0].location_id, Some(500));
    assert_eq!(deliveries[0].comment.as_deref(), Some("pausing"));

    // continue
    let msg = logic_continue_delivery(&mut deliveries, 1, Some("resume".into())).unwrap();
    assert_eq!(msg, "continued");
    assert_eq!(deliveries[0].status, models::DeliveryStatus::InProgress);
    assert_eq!(deliveries[0].location_id, None);
    assert_eq!(deliveries[0].comment.as_deref(), Some("resume"));

    // complete
    let msg = logic_make_delivery(&orders, &mut deliveries, 1).unwrap();
    assert_eq!(msg, "completed");
    assert_eq!(deliveries[0].status, models::DeliveryStatus::COMPLETE);
    assert!(deliveries[0].ended_at.is_some());
    assert_eq!(deliveries[0].location_id, Some(100)); // moved to client
}

#[test]
fn transitions_error_paths() {
    let orders = vec![order(1, 100, 200)];
    let mut deliveries: Vec<models::Delivery> = vec![];

    // cannot store when no active
    assert!(logic_store_delivery(&mut deliveries, 1, 500, None).is_err());
    // cannot continue when no stored
    assert!(logic_continue_delivery(&mut deliveries, 1, None).is_err());
    // cannot fail when no active
    assert!(logic_fail_delivery(&orders, &mut deliveries, 1, None).is_err());
    // cannot lose when no active
    assert!(logic_lose_delivery(&mut deliveries, 1, None).is_err());

    // take -> store -> continue
    logic_take_order(&orders, &mut deliveries, 1).unwrap();
    logic_store_delivery(&mut deliveries, 1, 9, None).unwrap();
    // cannot store when stored
    assert!(logic_store_delivery(&mut deliveries, 1, 9, None).is_err());
    // continue back to in-progress
    logic_continue_delivery(&mut deliveries, 1, None).unwrap();
    // cannot continue when in-progress
    assert!(logic_continue_delivery(&mut deliveries, 1, None).is_err());
}

#[test]
fn bulk_actions_edge_cases() {
    let orders = vec![order(1, 100, 200), order(2, 101, 201), order(3, 102, 202)];
    let mut deliveries: Vec<models::Delivery> = vec![];

    // Accept duplicates should not create extras
    let msg = logic_bulk_accept(&orders, &mut deliveries, &[1, 2, 1]).unwrap();
    assert_eq!(msg, "accepted 2");
    assert_eq!(deliveries.len(), 2);

    // Accept including unknown order 999 is ignored
    let msg = logic_bulk_accept(&orders, &mut deliveries, &[2, 3, 999]).unwrap();
    assert_eq!(msg, "accepted 1"); // only 3 added
    assert_eq!(deliveries.len(), 3);

    // Bulk complete: if no delivery exists for 1..3 it's fine; for new number creates then completes
    let msg = logic_bulk_complete(&orders, &mut deliveries, &[1, 2, 3]).unwrap();
    assert_eq!(msg, "completed 3");
    assert!(
        deliveries
            .iter()
            .all(|d| matches!(d.status, models::DeliveryStatus::COMPLETE))
    );

    // Bulk complete also creates when missing
    let mut deliveries: Vec<models::Delivery> = vec![];
    let msg = logic_bulk_complete(&orders, &mut deliveries, &[2]).unwrap();
    assert_eq!(msg, "completed 1");
    assert_eq!(deliveries.len(), 1);
    assert_eq!(deliveries[0].order_number, 2);
    assert_eq!(deliveries[0].status, models::DeliveryStatus::COMPLETE);
}
