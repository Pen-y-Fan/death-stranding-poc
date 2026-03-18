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

test('status_transitions_happy_paths', () => {
    localStorage.clear();
    const orders = [order(1, 100, 200)];
    poc.import_orders(JSON.stringify(orders));

    // take
    let msg = poc.take_order(1);
    assert.strictEqual(msg, "order 1 taken");
    let deliveries = JSON.parse(poc.export_deliveries());
    assert.strictEqual(deliveries.length, 1);
    assert.strictEqual(deliveries[0].status, "InProgress");
    assert.ok(deliveries[0].started_at);

    // store
    msg = poc.store_delivery(1, 500, "pausing");
    assert.strictEqual(msg, "stored");
    deliveries = JSON.parse(poc.export_deliveries());
    assert.strictEqual(deliveries[0].status, "STORED");
    assert.strictEqual(deliveries[0].location_id, 500);
    assert.strictEqual(deliveries[0].comment, "pausing");

    // continue
    msg = poc.continue_delivery(1, "resume");
    assert.strictEqual(msg, "continued");
    deliveries = JSON.parse(poc.export_deliveries());
    assert.strictEqual(deliveries[0].status, "InProgress");
    assert.strictEqual(deliveries[0].location_id, null);
    assert.strictEqual(deliveries[0].comment, "resume");

    // complete
    msg = poc.make_delivery(1);
    assert.strictEqual(msg, "completed");
    deliveries = JSON.parse(poc.export_deliveries());
    assert.strictEqual(deliveries[0].status, "COMPLETE");
    assert.ok(deliveries[0].ended_at);
    assert.strictEqual(deliveries[0].location_id, 100); // moved to client
});

test('transitions_error_paths', () => {
    localStorage.clear();
    const orders = [order(1, 100, 200)];
    poc.import_orders(JSON.stringify(orders));

    // cannot store when no active
    assert.throws(() => poc.store_delivery(1, 500, null), /delivery not found/);
    // cannot continue when no stored
    assert.throws(() => poc.continue_delivery(1, null), /delivery not found/);
    // cannot fail when no active
    assert.throws(() => poc.fail_delivery(1, null), /delivery not found/);
    // cannot lose when no active
    assert.throws(() => poc.lose_delivery(1, null), /delivery not found/);

    // take -> store -> continue
    poc.take_order(1);
    poc.store_delivery(1, 9, null);
    // cannot store when stored
    assert.throws(() => poc.store_delivery(1, 9, null), /can only store an in-progress delivery/);
    // continue back to in-progress
    poc.continue_delivery(1, null);
    // cannot continue when in-progress
    assert.throws(() => poc.continue_delivery(1, null), /can only continue a stored delivery/);
});

test('bulk_actions_edge_cases', () => {
    localStorage.clear();
    const orders = [order(1, 100, 200), order(2, 101, 201), order(3, 102, 202)];
    poc.import_orders(JSON.stringify(orders));

    // Accept duplicates should not create extras
    let msg = poc.bulk_accept([1, 2, 1]);
    assert.strictEqual(msg, "accepted 2");
    let deliveries = JSON.parse(poc.export_deliveries());
    assert.strictEqual(deliveries.length, 2);

    // Accept including unknown order 999 is ignored
    msg = poc.bulk_accept([2, 3, 999]);
    assert.strictEqual(msg, "accepted 1"); // only 3 added
    deliveries = JSON.parse(poc.export_deliveries());
    assert.strictEqual(deliveries.length, 3);

    // Bulk complete: if no delivery exists for 1..3 it's fine; for new number creates then completes
    msg = poc.bulk_complete([1, 2, 3]);
    assert.strictEqual(msg, "completed 3");
    deliveries = JSON.parse(poc.export_deliveries());
    assert.ok(deliveries.every(d => d.status === "COMPLETE"));

    // Bulk complete also creates when missing
    localStorage.clear();
    poc.import_orders(JSON.stringify(orders));
    msg = poc.bulk_complete([2]);
    assert.strictEqual(msg, "completed 1");
    deliveries = JSON.parse(poc.export_deliveries());
    assert.strictEqual(deliveries.length, 1);
    assert.strictEqual(deliveries[0].order_number, 2);
    assert.strictEqual(deliveries[0].status, "COMPLETE");
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

test('multiple_deliveries_same_order_sequential_ids', () => {
    localStorage.clear();
    const orders = [order(229, 100, 200)];
    poc.import_orders(JSON.stringify(orders));

    // First take of order 229
    poc.take_order(229);
    let deliveries = JSON.parse(poc.export_deliveries());
    assert.strictEqual(deliveries.length, 1);
    assert.strictEqual(deliveries[0].order_number, 229);
    assert.strictEqual(deliveries[0].id, 1);

    // Complete it
    poc.make_delivery(229);
    deliveries = JSON.parse(poc.export_deliveries());
    assert.strictEqual(deliveries[0].status, "COMPLETE");

    // Second take of order 229
    poc.take_order(229);
    deliveries = JSON.parse(poc.export_deliveries());
    assert.strictEqual(deliveries.length, 2);
    assert.strictEqual(deliveries[1].order_number, 229);
    assert.strictEqual(deliveries[1].id, 2); // Sequential ID

    // Complete it too
    poc.make_delivery(229);
    deliveries = JSON.parse(poc.export_deliveries());
    assert.strictEqual(deliveries[1].status, "COMPLETE");

    // Third take of order 229
    poc.take_order(229);
    deliveries = JSON.parse(poc.export_deliveries());
    assert.strictEqual(deliveries.length, 3);
    assert.strictEqual(deliveries[2].order_number, 229);
    assert.strictEqual(deliveries[2].id, 3); // Sequential ID
});
