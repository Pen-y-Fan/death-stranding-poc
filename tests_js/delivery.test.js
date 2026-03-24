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

test('delete_delivery', () => {
    localStorage.clear();
    const orders = [order(1, 100, 200)];
    poc.import_orders(JSON.stringify(orders));
    poc.take_order(1);
    
    let deliveries = JSON.parse(poc.export_deliveries());
    assert.strictEqual(deliveries.length, 1);
    const id = deliveries[0].id;

    // Delete existing
    const res = poc.delete_delivery(id);
    assert.strictEqual(res, true);
    deliveries = JSON.parse(poc.export_deliveries());
    assert.strictEqual(deliveries.length, 0);

    // Delete non-existing
    const res2 = poc.delete_delivery(id);
    assert.strictEqual(res2, false);
});

test('bulk_delete_deliveries', () => {
    localStorage.clear();
    const orders = [order(1, 100, 200), order(2, 101, 201), order(3, 102, 202)];
    poc.import_orders(JSON.stringify(orders));
    poc.take_order(1);
    poc.take_order(2);
    poc.take_order(3);

    let deliveries = JSON.parse(poc.export_deliveries());
    assert.strictEqual(deliveries.length, 3);
    const ids = deliveries.map(d => d.id);

    // Bulk delete two
    const count = poc.bulk_delete_deliveries([ids[0], ids[1]]);
    assert.strictEqual(count, 2);
    deliveries = JSON.parse(poc.export_deliveries());
    assert.strictEqual(deliveries.length, 1);
    assert.strictEqual(deliveries[0].id, ids[2]);

    // Bulk delete with some non-existing
    const count2 = poc.bulk_delete_deliveries([ids[2], 999]);
    assert.strictEqual(count2, 1);
    deliveries = JSON.parse(poc.export_deliveries());
    assert.strictEqual(deliveries.length, 0);
});

test('update_delivery', () => {
    localStorage.clear();
    const orders = [order(1, 100, 200)];
    poc.import_orders(JSON.stringify(orders));
    poc.take_order(1);

    let deliveries = JSON.parse(poc.export_deliveries());
    const id = deliveries[0].id;

    // Update status and comment
    const res = poc.update_delivery(id, 'COMPLETE', 'Success');
    assert.strictEqual(res, true);
    deliveries = JSON.parse(poc.export_deliveries());
    assert.strictEqual(deliveries[0].status, 'COMPLETE');
    assert.strictEqual(deliveries[0].comment, 'Success');
    assert.ok(deliveries[0].updated_at);

    // Update only status
    poc.update_delivery(id, 'FAILED');
    deliveries = JSON.parse(poc.export_deliveries());
    assert.strictEqual(deliveries[0].status, 'FAILED');
    assert.strictEqual(deliveries[0].comment, 'Success'); // preserved

    // Update only comment
    poc.update_delivery(id, null, 'Retry');
    deliveries = JSON.parse(poc.export_deliveries());
    assert.strictEqual(deliveries[0].status, 'FAILED'); // preserved
    assert.strictEqual(deliveries[0].comment, 'Retry');

    // Update non-existing
    const res2 = poc.update_delivery(999, 'COMPLETE', 'Non-existent');
    assert.strictEqual(res2, false);
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

test('delete_delivery_numeric_vs_string_id', () => {
    localStorage.clear();
    const orders = [order(1, 100, 200)];
    poc.import_orders(JSON.stringify(orders));
    poc.take_order(1);
    
    let deliveries = JSON.parse(poc.export_deliveries());
    const id = deliveries[0].id; // Numeric 1

    // Delete with string ID
    const res = poc.delete_delivery(String(id));
    assert.strictEqual(res, true);
    deliveries = JSON.parse(poc.export_deliveries());
    assert.strictEqual(deliveries.length, 0);

    // Re-add
    poc.take_order(1); // ID will be 2
    deliveries = JSON.parse(poc.export_deliveries());
    const id2 = deliveries[0].id;

    // Delete with numeric ID
    const res2 = poc.delete_delivery(id2);
    assert.strictEqual(res2, true);
    deliveries = JSON.parse(poc.export_deliveries());
    assert.strictEqual(deliveries.length, 0);
});
