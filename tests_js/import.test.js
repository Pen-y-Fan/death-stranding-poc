import './setup.js';
import { test } from 'node:test';
import assert from 'node:assert';
import * as poc from '../js/death_stranding_poc.js';

function order(n, client, dest) {
    return {
        number: n,
        name: `Order ${n}`,
        client_id: client,
        destination_id: dest,
        delivery_category_id: 1,
        max_likes: 0.0,
        weight: 0.0,
    };
}

test('validate_orders_ok', () => {
    localStorage.clear();
    const orders = [
        {
            number: 100,
            name: "Test",
            client_id: 1,
            destination_id: 2,
            delivery_category_id: 1,
            max_likes: 1.0,
            weight: 0.0,
        },
        {
            number: 101,
            name: "Another",
            client_id: 1,
            destination_id: 2,
            delivery_category_id: 2,
            max_likes: 5.0,
            weight: 10.0,
        },
    ];
    assert.doesNotThrow(() => poc.import_orders(JSON.stringify(orders)));
});

test('validate_orders_duplicate_number', () => {
    localStorage.clear();
    const orders = [
        {
            number: 100,
            name: "A",
            client_id: 1,
            destination_id: 2,
            delivery_category_id: 1,
            max_likes: 1.0,
            weight: 0.0,
        },
        {
            number: 100,
            name: "B",
            client_id: 1,
            destination_id: 2,
            delivery_category_id: 2,
            max_likes: 5.0,
            weight: 10.0,
        },
    ];
    assert.throws(() => poc.import_orders(JSON.stringify(orders)), /Duplicate order number: 100/);
});

test('import_deliveries_filtering', () => {
    localStorage.clear();
    const orders = [order(100, 1, 2), order(101, 3, 4)];
    poc.import_orders(JSON.stringify(orders));

    const incoming = [
        {
            id: 1,
            order_number: 100,
            status: "InProgress",
            location_id: null,
            started_at: null,
            ended_at: null,
            comment: null,
            user_id: null,
        },
        {
            id: 2,
            order_number: 999, // Invalid
            status: "InProgress",
            location_id: null,
            started_at: null,
            ended_at: null,
            comment: null,
            user_id: null,
        },
    ];

    poc.import_deliveries(JSON.stringify(incoming));
    const filtered = JSON.parse(poc.export_deliveries());
    assert.strictEqual(filtered.length, 1);
    assert.strictEqual(filtered[0].order_number, 100);
});
