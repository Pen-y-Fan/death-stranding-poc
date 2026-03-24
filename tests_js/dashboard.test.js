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

function location(id, district_id, name) {
    return {
        id: id,
        district_id: district_id,
        name: name,
        category_id: 1,
        lat: 0.0,
        lng: 0.0
    };
}

function district(id, region) {
    return {
        id: id,
        region: region,
        name: `District ${id}`
    };
}

test('get_dashboard_summary_unique_completed_orders', () => {
    localStorage.clear();
    
    // Setup orders, locations, and districts for dashboard summary
    const orders = [
        order(1, 101, 201), // East
        order(2, 101, 201), // East (duplicate destination)
        order(3, 102, 202), // West
        order(4, 103, 203), // Central AE
        order(5, 104, 204), // Central FM
        order(6, 105, 205), // Central NW
    ];
    poc.import_orders(JSON.stringify(orders));

    const districtsList = [
        district(1, 'East'),
        district(2, 'West'),
        district(3, 'Central'),
    ];
    
    poc.import_districts(JSON.stringify(districtsList));
    
    poc.import_locations(JSON.stringify([
        location(101, 1, 'Source Port'),
        location(102, 2, 'West Base'),
        location(103, 3, 'Abby Base'),
        location(104, 3, 'Frank Base'),
        location(105, 3, 'North Base'),
        location(201, 1, 'East Port'),
        location(202, 2, 'West Knot'),
        location(203, 3, 'Abby Knot'), // AE
        location(204, 3, 'Frank Knot'), // FM
        location(205, 3, 'North Knot'), // NW
    ]));

    // Create deliveries: multiple completed deliveries for order 1
    poc.take_order(1);
    poc.make_delivery(1); // One complete for order 1
    
    // Another take and complete for order 1 (sequential ID 2)
    poc.take_order(1);
    poc.make_delivery(1);

    // Complete order 3 (West)
    poc.take_order(3);
    poc.make_delivery(3);

    // Complete order 4 (Central AE)
    poc.take_order(4);
    poc.make_delivery(4);

    // Complete order 5 (Central FM)
    poc.take_order(5);
    poc.make_delivery(5);

    // Complete order 6 (Central NW)
    poc.take_order(6);
    poc.make_delivery(6);

    const summaryJson = poc.get_dashboard_summary();
    const summary = JSON.parse(summaryJson);

    // Verification of unique completed order counts
    assert.strictEqual(summary.total_orders, 6);
    // Order 1 completed twice, but should only count as 1.
    // Order 3, 4, 5, 6 each completed once.
    // Total completed = 5 (1, 3, 4, 5, 6)
    assert.strictEqual(summary.completed_orders_total, 5);

    // Verify incoming and outgoing counts
    // Order 1 (101->201): Outgoing from Source Port, Incoming to East Port
    assert.strictEqual(summary.locations['East Port'].completed_in, 1);
    assert.strictEqual(summary.locations['Source Port'].completed_out, 1);

    // Order 3 (102->202): Outgoing from West Base, Incoming to West Knot
    assert.strictEqual(summary.locations['West Knot'].completed_in, 1);
    assert.strictEqual(summary.locations['West Base'].completed_out, 1);

    assert.strictEqual(summary.locations['Abby Knot'].completed_in, 1);
    assert.strictEqual(summary.locations['Frank Knot'].completed_in, 1);
    assert.strictEqual(summary.locations['North Knot'].completed_in, 1);
    
    // Check that West was mapped to East for display
    assert.strictEqual(summary.locations['West Knot'].region, 'East'); 
    assert.strictEqual(summary.status_counts.COMPLETE, 6); // 2 for order 1 + 3, 4, 5, 6
});
