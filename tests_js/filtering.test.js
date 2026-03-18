import './setup.js';
import { test } from 'node:test';
import assert from 'node:assert';
import * as poc from '../js/death_stranding_poc.js';

function sample_orders() {
    return [
        {
            number: 1,
            name: "A",
            client_id: 10,
            destination_id: 20,
            delivery_category_id: 100,
            max_likes: 5.0,
            weight: 2.0,
        },
        {
            number: 2,
            name: "B",
            client_id: 11,
            destination_id: 21,
            delivery_category_id: 101,
            max_likes: 3.0,
            weight: 5.0,
        },
        {
            number: 3,
            name: "Cargo",
            client_id: 12,
            destination_id: 20,
            delivery_category_id: 100,
            max_likes: 9.0,
            weight: 1.0,
        },
    ];
}

function sample_locations() {
    return [
        { id: 10, name: "L10", district_id: 1, is_physical: true },
        { id: 11, name: "L11", district_id: 1, is_physical: true },
        { id: 12, name: "L12", district_id: 2, is_physical: true },
        { id: 20, name: "L20", district_id: 3, is_physical: true },
        { id: 21, name: "L21", district_id: 2, is_physical: true },
    ];
}

function sample_deliveries() {
    return [
        {
            id: 1,
            order_number: 1,
            status: "InProgress",
            location_id: null,
            started_at: null,
            ended_at: null,
            comment: null,
            user_id: 1,
        },
        {
            id: 2,
            order_number: 2,
            status: "COMPLETE",
            location_id: null,
            started_at: null,
            ended_at: null,
            comment: null,
            user_id: 1,
        },
    ];
}

test('filter_by_district', () => {
    localStorage.clear();
    poc.import_orders(JSON.stringify(sample_orders()));
    poc.import_locations(JSON.stringify(sample_locations()));
    poc.import_deliveries(JSON.stringify(sample_deliveries()));

    const result = JSON.parse(poc.query_orders(JSON.stringify({ district_id: 1 }), 1, 10, 'number', 'asc'));
    const nums = result.items.map(i => i.number);
    assert.deepStrictEqual(nums, [1, 2]); // orders with client in district 1
});

test('filter_by_category_and_completion', () => {
    localStorage.clear();
    poc.import_orders(JSON.stringify(sample_orders()));
    poc.import_locations(JSON.stringify(sample_locations()));
    poc.import_deliveries(JSON.stringify(sample_deliveries()));

    // completion: false means not COMPLETE
    const result = JSON.parse(poc.query_orders(JSON.stringify({ delivery_category_id: 100, completion: false }), 1, 10, 'number', 'asc'));
    const nums = result.items.map(i => i.number);
    assert.deepStrictEqual(nums, [1, 3]);
});

test('sort_and_search_and_paginate', () => {
    localStorage.clear();
    poc.import_orders(JSON.stringify(sample_orders()));
    poc.import_deliveries(JSON.stringify(sample_deliveries()));

    // Sort by weight Asc
    let result = JSON.parse(poc.query_orders(null, 1, 10, 'weight', 'asc'));
    let weights = result.items.map(i => i.weight);
    assert.deepStrictEqual(weights, [1.0, 2.0, 5.0]);

    // Search
    result = JSON.parse(poc.query_orders(null, 1, 10, 'number', 'asc', 'car'));
    assert.strictEqual(result.items.length, 1);
    assert.strictEqual(result.items[0].name, "Cargo");

    // Paginate: page 2, per_page 2
    result = JSON.parse(poc.query_orders(null, 2, 2, 'number', 'asc'));
    assert.strictEqual(result.total, 3);
    assert.strictEqual(result.items.length, 1);
    assert.strictEqual(result.items[0].number, 3); // A(1), B(2) are first page, Cargo(3) is second
});
