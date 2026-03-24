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

test('fail_and_lose_delivery', () => {
    localStorage.clear();
    const orders = [order(1, 100, 200)];
    poc.import_orders(JSON.stringify(orders));
    
    // Fail
    poc.take_order(1);
    const msgFail = poc.fail_delivery(1, 'Broken');
    assert.strictEqual(msgFail, 'failed');
    let deliveries = JSON.parse(poc.export_deliveries());
    assert.strictEqual(deliveries[0].status, 'FAILED');
    assert.strictEqual(deliveries[0].comment, 'Broken');
    assert.strictEqual(deliveries[0].location_id, 200); // Destination
    assert.ok(deliveries[0].ended_at);

    // Lose
    poc.take_order(1); // ID 2
    const msgLose = poc.lose_delivery(1, 'Lost in void');
    assert.strictEqual(msgLose, 'lost');
    deliveries = JSON.parse(poc.export_deliveries());
    assert.strictEqual(deliveries[1].status, 'LOST');
    assert.strictEqual(deliveries[1].comment, 'Lost in void');
    assert.ok(deliveries[1].ended_at);
});

test('bulk_complete_deliveries', () => {
    localStorage.clear();
    const orders = [order(1, 100, 200), order(2, 101, 201), order(3, 102, 202)];
    poc.import_orders(JSON.stringify(orders));

    // Bulk complete: if no delivery exists for 1..3 it's fine; for new number creates then completes
    let msg = poc.bulk_complete([1, 2, 3]);
    assert.strictEqual(msg, "completed 3");
    let deliveries = JSON.parse(poc.export_deliveries());
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
