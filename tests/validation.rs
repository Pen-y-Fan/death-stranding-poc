use death_stranding_poc::models::{Order, validate_orders};

#[test]
fn validate_orders_ok() {
    let orders = vec![
        Order {
            number: 100,
            name: "Test".into(),
            client_id: 1,
            destination_id: 2,
            delivery_category_id: 1,
            max_likes: 1.0,
            weight: 0.0,
        },
        Order {
            number: 101,
            name: "Another".into(),
            client_id: 1,
            destination_id: 2,
            delivery_category_id: 2,
            max_likes: 5.0,
            weight: 10.0,
        },
    ];
    assert!(validate_orders(&orders).is_ok());
}

#[test]
fn validate_orders_duplicate_number() {
    let orders = vec![
        Order {
            number: 100,
            name: "A".into(),
            client_id: 1,
            destination_id: 2,
            delivery_category_id: 1,
            max_likes: 1.0,
            weight: 0.0,
        },
        Order {
            number: 100,
            name: "B".into(),
            client_id: 1,
            destination_id: 2,
            delivery_category_id: 2,
            max_likes: 5.0,
            weight: 10.0,
        },
    ];
    assert!(validate_orders(&orders).is_err());
}
