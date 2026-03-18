import './setup.js';
import { test } from 'node:test';
import assert from 'node:assert';
import * as poc from '../js/death_stranding_poc.js';

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
    // In JS, import_orders performs validation and throws if invalid
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
