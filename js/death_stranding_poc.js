// Hand-written JS implementation of the Rust logic for the Death Stranding PoC.
// This replaces the WASM module to avoid recurring RangeError: WebAssembly.Table.grow issues in browsers.

// --- Storage Keys (Namespaced) ---
const KEY_SCHEMA_VERSION = "ds:schema_version";
const KEY_DISTRICTS = "ds:districts";
const KEY_LOCATIONS = "ds:locations";
const KEY_CATEGORIES = "ds:delivery_categories";
const KEY_ORDERS = "ds:orders";
const KEY_DELIVERIES = "ds:deliveries";

// --- Enums ---
const DeliveryStatus = {
    InProgress: "InProgress",
    STORED: "STORED",
    COMPLETE: "COMPLETE",
    FAILED: "FAILED",
    LOST: "LOST"
};

// --- Helper Functions ---
function now_iso_utc() {
    return new Date().toISOString();
}

function get_storage_item(key, defaultValue = "[]") {
    try {
        const val = localStorage.getItem(key);
        return val !== null ? val : defaultValue;
    } catch (e) {
        console.error(`Failed to read ${key} from localStorage`, e);
        return defaultValue;
    }
}

function set_storage_item(key, value) {
    try {
        localStorage.setItem(key, value);
        return true;
    } catch (e) {
        console.error(`Failed to write ${key} to localStorage`, e);
        return false;
    }
}

function load_list(key) {
    try {
        return JSON.parse(get_storage_item(key, "[]"));
    } catch (e) {
        console.error(`Failed to parse ${key}`, e);
        return [];
    }
}

function save_list(key, list) {
    return set_storage_item(key, JSON.stringify(list));
}

function validate_nonzero(name, value) {
    if (value === 0 || value === null || value === undefined) {
        throw new Error(`${name} must be > 0`);
    }
}

function validate_comment(comment) {
    if (comment && comment.length > 500) {
        throw new Error("comment too long (max 500 chars)");
    }
}

// --- Named Exports (Matching Rust/WASM API) ---

export function set_schema_version(version) {
    set_storage_item(KEY_SCHEMA_VERSION, version);
}

export function get_schema_version() {
    return localStorage.getItem(KEY_SCHEMA_VERSION) || "";
}

export function import_districts(json) {
    try {
        const districts = JSON.parse(json);
        save_list(KEY_DISTRICTS, districts);
        set_schema_version("1");
    } catch (e) {
        throw new Error(`invalid JSON array: ${e.message}`);
    }
}

export function import_locations(json) {
    const locations = JSON.parse(json);
    save_list(KEY_LOCATIONS, locations);
    set_schema_version("1");
}

export function import_delivery_categories(json) {
    const cats = JSON.parse(json);
    save_list(KEY_CATEGORIES, cats);
    set_schema_version("1");
}

export function import_orders(json) {
    const orders = JSON.parse(json);
    // Basic validation: unique numbers
    const seen = new Set();
    for (const o of orders) {
        if (o.number === 0) throw new Error(`Order has number 0 (invalid)`);
        if (seen.has(o.number)) throw new Error(`Duplicate order number: ${o.number}`);
        seen.add(o.number);
    }
    save_list(KEY_ORDERS, orders);
    set_schema_version("1");
}

export function get_districts() {
    return get_storage_item(KEY_DISTRICTS, "[]");
}

export function get_locations() {
    return get_storage_item(KEY_LOCATIONS, "[]");
}

export function get_location(id) {
    const list = load_list(KEY_LOCATIONS);
    const loc = list.find(l => l.id === id);
    return loc ? JSON.stringify(loc) : "{}";
}

export function get_district(id) {
    const list = load_list(KEY_DISTRICTS);
    const d = list.find(it => it.id === id);
    return d ? JSON.stringify(d) : "{}";
}

export function get_delivery_categories() {
    return get_storage_item(KEY_CATEGORIES, "[]");
}

export function get_orders() {
    return get_storage_item(KEY_ORDERS, "[]");
}

export function import_deliveries(json) {
    try {
        const deliveries = JSON.parse(json);
        const orders = load_list(KEY_ORDERS);
        let filtered = deliveries;
        if (orders.length > 0) {
            const orderNums = new Set(orders.map(o => o.number));
            filtered = deliveries.filter(d => orderNums.has(d.order_number || d.order_id));
        }
        save_list(KEY_DELIVERIES, filtered);
    } catch (e) {
        throw new Error(`invalid JSON array: ${e.message}`);
    }
}

export function export_deliveries() {
    return get_storage_item(KEY_DELIVERIES, "[]");
}

function has_active_or_stored(deliveries, order_number) {
    return deliveries.some(d => d.order_number === order_number && (d.status === DeliveryStatus.InProgress || d.status === DeliveryStatus.STORED));
}

function get_active_delivery(deliveries, order_number) {
    return deliveries.find(d => d.order_number === order_number && (d.status === DeliveryStatus.InProgress || d.status === DeliveryStatus.STORED));
}

function make_new_delivery(deliveries, order_number, status) {
    const next_id = deliveries.length > 0 ? Math.max(...deliveries.map(d => d.id)) + 1 : 1;
    return {
        id: next_id,
        order_number,
        status,
        location_id: null,
        started_at: now_iso_utc(),
        ended_at: null,
        comment: null,
        user_id: 1
    };
}

export function take_order(order_number) {
    validate_nonzero("order_number", order_number);
    const orders = load_list(KEY_ORDERS);
    const order = orders.find(o => o.number === order_number);
    if (!order) throw new Error(`order ${order_number} not found`);

    const deliveries = load_list(KEY_DELIVERIES);
    if (has_active_or_stored(deliveries, order_number)) {
        throw new Error("delivery already active or stored for this order");
    }

    deliveries.push(make_new_delivery(deliveries, order_number, DeliveryStatus.InProgress));
    save_list(KEY_DELIVERIES, deliveries);
    return `order ${order_number} taken`;
}

export function store_delivery(order_number, location_id, comment) {
    validate_nonzero("order_number", order_number);
    validate_nonzero("location_id", location_id);
    validate_comment(comment);

    const deliveries = load_list(KEY_DELIVERIES);
    const d = get_active_delivery(deliveries, order_number);
    if (!d) throw new Error("delivery not found");

    if (d.status !== DeliveryStatus.InProgress) {
        throw new Error("can only store an in-progress delivery");
    }

    d.status = DeliveryStatus.STORED;
    d.location_id = location_id;
    if (comment) d.comment = comment;

    save_list(KEY_DELIVERIES, deliveries);
    return "stored";
}

export function continue_delivery(order_number, comment) {
    validate_nonzero("order_number", order_number);
    validate_comment(comment);

    const deliveries = load_list(KEY_DELIVERIES);
    const d = get_active_delivery(deliveries, order_number);
    if (!d) throw new Error("delivery not found");

    if (d.status !== DeliveryStatus.STORED) {
        throw new Error("can only continue a stored delivery");
    }

    d.status = DeliveryStatus.InProgress;
    d.location_id = null;
    if (comment) d.comment = comment;

    save_list(KEY_DELIVERIES, deliveries);
    return "continued";
}

export function make_delivery(order_number) {
    validate_nonzero("order_number", order_number);
    const orders = load_list(KEY_ORDERS);
    const order = orders.find(o => o.number === order_number);
    if (!order) throw new Error("order not found");

    const deliveries = load_list(KEY_DELIVERIES);
    let d = get_active_delivery(deliveries, order_number);
    if (!d) {
        d = make_new_delivery(deliveries, order_number, DeliveryStatus.InProgress);
        deliveries.push(d);
    }

    d.status = DeliveryStatus.COMPLETE;
    d.ended_at = now_iso_utc();
    d.location_id = order.client_id;

    save_list(KEY_DELIVERIES, deliveries);
    return "completed";
}

export function fail_delivery(order_number, comment) {
    validate_nonzero("order_number", order_number);
    validate_comment(comment);

    const orders = load_list(KEY_ORDERS);
    const order = orders.find(o => o.number === order_number);
    if (!order) throw new Error("order not found");

    const deliveries = load_list(KEY_DELIVERIES);
    const d = get_active_delivery(deliveries, order_number);
    if (!d) throw new Error("delivery not found");

    d.status = DeliveryStatus.FAILED;
    d.ended_at = now_iso_utc();
    d.location_id = order.destination_id;
    if (comment) d.comment = comment;

    save_list(KEY_DELIVERIES, deliveries);
    return "failed";
}

export function lose_delivery(order_number, comment) {
    validate_nonzero("order_number", order_number);
    validate_comment(comment);

    const deliveries = load_list(KEY_DELIVERIES);
    const d = get_active_delivery(deliveries, order_number);
    if (!d) throw new Error("delivery not found");

    d.status = DeliveryStatus.LOST;
    d.ended_at = now_iso_utc();
    if (comment) d.comment = comment;

    save_list(KEY_DELIVERIES, deliveries);
    return "lost";
}

export function bulk_accept(order_numbers) {
    if (!Array.isArray(order_numbers)) {
         order_numbers = Array.from(order_numbers);
    }
    if (order_numbers.some(n => n === 0)) throw new Error("order_numbers contain invalid 0 value");

    const orders = load_list(KEY_ORDERS);
    const deliveries = load_list(KEY_DELIVERIES);
    let created = 0;

    for (const n of order_numbers) {
        if (!orders.some(o => o.number === n)) continue;
        if (!has_active_or_stored(deliveries, n)) {
            deliveries.push(make_new_delivery(deliveries, n, DeliveryStatus.InProgress));
            created++;
        }
    }
    save_list(KEY_DELIVERIES, deliveries);
    return `accepted ${created}`;
}

export function bulk_complete(order_numbers) {
    if (!Array.isArray(order_numbers)) order_numbers = Array.from(order_numbers);
    if (order_numbers.some(n => n === 0)) throw new Error("order_numbers contain invalid 0 value");

    const orders = load_list(KEY_ORDERS);
    const deliveries = load_list(KEY_DELIVERIES);
    let completed = 0;

    for (const n of order_numbers) {
        const order = orders.find(o => o.number === n);
        if (!order) continue;

        let d = get_active_delivery(deliveries, n);
        if (!d) {
             d = make_new_delivery(deliveries, n, DeliveryStatus.InProgress);
             deliveries.push(d);
        }
        d.status = DeliveryStatus.COMPLETE;
        d.ended_at = now_iso_utc();
        d.location_id = order.client_id;
        completed++;
    }
    save_list(KEY_DELIVERIES, deliveries);
    return `completed ${completed}`;
}

export function query_orders(filter_json, page, per_page, sort_key, sort_dir, search) {
    const orders = load_list(KEY_ORDERS);
    const deliveries = load_list(KEY_DELIVERIES);
    const locations = load_list(KEY_LOCATIONS);

    const filter = filter_json ? JSON.parse(filter_json) : {};

    let filtered = orders;

    // Filter by district
    if (filter.district_id) {
        const locIds = new Set(locations.filter(l => l.district_id === filter.district_id).map(l => l.id));
        filtered = filtered.filter(o => locIds.has(o.client_id) || locIds.has(o.destination_id));
    }

    if (filter.client_id) filtered = filtered.filter(o => o.client_id === filter.client_id);
    if (filter.destination_id) filtered = filtered.filter(o => o.destination_id === filter.destination_id);
    if (filter.delivery_category_id) filtered = filtered.filter(o => o.delivery_category_id === filter.delivery_category_id);

    if (filter.delivery_status) {
        const status = filter.delivery_status;
        if (status === "any") {
            filtered = filtered.filter(o => deliveries.some(d => d.order_number === o.number));
        } else if (status === "none") {
            filtered = filtered.filter(o => !deliveries.some(d => d.order_number === o.number));
        } else {
            const mapped = status.toLowerCase() === "in_progress" ? DeliveryStatus.InProgress : status.toUpperCase();
            filtered = filtered.filter(o => deliveries.some(d => d.order_number === o.number && d.status === mapped));
        }
    }

    if (filter.completion !== undefined && filter.completion !== null) {
        if (filter.completion === true || filter.completion === "true") {
            filtered = filtered.filter(o => deliveries.some(d => d.order_number === o.number && d.status === DeliveryStatus.COMPLETE));
        } else {
            filtered = filtered.filter(o => !deliveries.some(d => d.order_number === o.number && d.status === DeliveryStatus.COMPLETE));
        }
    }

    // Map to OrderListItem
    let items = filtered.map(o => {
        let user_dels = deliveries.filter(d => d.order_number === o.number && (d.user_id === 1 || !d.user_id));
        let active = user_dels.find(d => d.status === DeliveryStatus.InProgress || d.status === DeliveryStatus.STORED);
        let has_complete = user_dels.some(d => d.status === DeliveryStatus.COMPLETE);
        let has_failed = user_dels.some(d => d.status === DeliveryStatus.FAILED);
        let has_lost = user_dels.some(d => d.status === DeliveryStatus.LOST);

        let current_status = null;
        if (active) current_status = active.status;
        else if (has_complete) current_status = DeliveryStatus.COMPLETE;
        else if (has_failed) current_status = DeliveryStatus.FAILED;
        else if (has_lost) current_status = DeliveryStatus.LOST;

        return {
            number: o.number,
            name: o.name,
            client_id: o.client_id,
            destination_id: o.destination_id,
            delivery_category_id: o.delivery_category_id,
            max_likes: o.max_likes,
            weight: o.weight,
            delivery_status: current_status,
            is_completed: has_complete
        };
    });

    // Sorting
    const key = sort_key || "number";
    const dir = sort_dir || "asc";
    items.sort((a, b) => {
        let valA = a[key];
        let valB = b[key];
        if (typeof valA === "string") {
            valA = valA.toLowerCase();
            valB = valB.toLowerCase();
        }
        if (valA < valB) return dir === "asc" ? -1 : 1;
        if (valA > valB) return dir === "asc" ? 1 : -1;
        return 0;
    });

    // Search
    if (search && search.trim()) {
        const q = search.toLowerCase();
        items = items.filter(i => i.name.toLowerCase().includes(q) || i.number.toString().includes(q));
    }

    // Pagination
    const total = items.length;
    const start = (Math.max(1, page) - 1) * per_page;
    const paginated = items.slice(start, start + per_page);

    return JSON.stringify({ total, items: paginated });
}

export function get_dashboard_summary() {
    const orders = load_list(KEY_ORDERS);
    const deliveries = load_list(KEY_DELIVERIES);
    const locations = load_list(KEY_LOCATIONS);
    const districts = load_list(KEY_DISTRICTS);

    const locById = new Map(locations.map(l => [l.id, l]));
    const distById = new Map(districts.map(d => [d.id, d]));
    const orderByNum = new Map(orders.map(o => [o.number, o]));

    const out = {
        central_total: 0,
        central_ae: 0,
        central_fm: 0,
        central_nw: 0,
        east: 0,
        west: 0
    };

    for (const d of deliveries) {
        if (d.status !== DeliveryStatus.COMPLETE) continue;
        const order = orderByNum.get(d.order_number);
        if (!order) continue;
        const loc = locById.get(order.destination_id);
        if (!loc) continue;
        const dist = distById.get(loc.district_id);
        if (!dist) continue;

        if (dist.region === "East") {
            out.east++;
        } else if (dist.region === "West") {
            out.west++;
        } else if (dist.region === "Central") {
            out.central_total++;
            const first = loc.name.trim().charAt(0).toUpperCase();
            if (first >= 'A' && first <= 'E') out.central_ae++;
            else if (first >= 'F' && first <= 'M') out.central_fm++;
            else if (first >= 'N' && first <= 'W') out.central_nw++;
        }
    }

    return JSON.stringify(out);
}

export function initialize() {
    console.log("PoC Logic initialized");
}

// Mock the default export (init function)
export default async function init() {
    console.log("PoC Logic loaded via JS");
    return {};
}

export function initSync() {
    return {};
}

// --- UI Logic (moved from index.html) ---

export function initUI() {
    console.log('Modules imported, calling initUI()...');

    const orderListDiv = document.getElementById('order-list');
    const statusDiv = document.getElementById('delivery-status');
    const filterDistrict = document.getElementById('filter-district');
    const filterClient = document.getElementById('filter-client');
    const filterDestination = document.getElementById('filter-destination');
    const filterCategory = document.getElementById('filter-category');
    const filterSearch = document.getElementById('filter-search');

    let currentPage = 1;
    const perPage = 20;

    function getOrders() {
        try {
            const raw = localStorage.getItem('ds:orders');
            return raw ? JSON.parse(raw) : [];
        } catch (e) {
            console.error('Failed to read orders from localStorage', e);
            return [];
        }
    }

    function getDeliveries() {
        try {
            const raw = localStorage.getItem('ds:deliveries');
            return raw ? JSON.parse(raw) : [];
        } catch (e) {
            console.error('Failed to read deliveries from localStorage', e);
            return [];
        }
    }

    function render() {
        const banner = document.getElementById('seed-banner');
        const hasOrders = getOrders().length > 0;
        if (banner) banner.hidden = hasOrders;
        // Use JS query for orders with minimal filter/sort/pagination
        const statusVal = (document.querySelector('input[name="status"]:checked')?.value || '').trim();
        const completionVal = (document.querySelector('input[name="completion"]:checked')?.value || '').trim();
        const searchVal = filterSearch.value || '';
        const f = {};
        if (filterDistrict.value) f.district_id = parseInt(filterDistrict.value, 10);
        if (filterClient.value) f.client_id = parseInt(filterClient.value, 10);
        if (filterDestination.value) f.destination_id = parseInt(filterDestination.value, 10);
        if (filterCategory.value) f.delivery_category_id = parseInt(filterCategory.value, 10);
        if (statusVal) f.delivery_status = statusVal;
        if (completionVal) f.completion = completionVal === 'true';
        const json = query_orders(JSON.stringify(f), currentPage, perPage, 'number', 'asc', searchVal);
        let parsed;
        try {
            parsed = JSON.parse(json);
        } catch (e) {
            parsed = {total: 0, items: []};
        }
        const items = parsed.items || [];
        const deliveries = getDeliveries();

        // Build a simple table
        const locationsList = (() => {
            try {
                return JSON.parse(get_locations());
            } catch {
                return [];
            }
        })();
        const byId = new Map(locationsList.map(l => [l.id, l]));
        let html = '<table border="1" cellpadding="6" cellspacing="0" role="grid" aria-label="Orders list">';
        html += '<thead><tr><th scope="col">#</th><th scope="col">Name</th><th scope="col">Client</th><th scope="col">Destination</th><th scope="col">Status</th><th scope="col">Action</th></tr></thead><tbody>';
        for (const it of items) {
            const o = {id: it.number, description: it.name};
            const deliveriesLocal = getDeliveries();
            const dels = deliveriesLocal.filter(dd => (dd.order_number ?? dd.order_id) === o.id);
            const norm = (s) => {
                if (!s) return '';
                const t = String(s);
                const lo = t.toLowerCase();
                if (lo === 'in progress' || lo === 'in_progress' || lo === 'inprogress') return 'In progress';
                return t.charAt(0).toUpperCase() + t.slice(1).toLowerCase();
            };
            const active = dels.find(dd => {
                const st = norm(dd.status);
                return st === 'In progress' || st === 'Stored';
            });
            const hasComplete = dels.some(dd => norm(dd.status).toUpperCase() === 'COMPLETE');
            const hasFailed = dels.some(dd => norm(dd.status).toUpperCase() === 'FAILED');
            const hasLost = dels.some(dd => norm(dd.status).toUpperCase() === 'LOST');

            const statusNorm = active ? norm(active.status)
                : hasComplete ? 'Complete'
                    : hasFailed ? 'Failed'
                        : hasLost ? 'Lost'
                            : 'Not taken';

            const client = byId.get(it.client_id);
            const destination = byId.get(it.destination_id);
            const clientCell = client ? `<a href="#" class="btn-link" data-show-location="${client.id}" data-kind="client">${client.name}</a>` : '';
            const destCell = destination ? `<a href="#" class="btn-link" data-show-location="${destination.id}" data-kind="destination">${destination.name}</a>` : '';

            let actionHtml = '';
            if (!active) {
                actionHtml = `<button data-action="take" data-id="${o.id}" type="button" aria-label="Take order ${o.id}">Start</button>`;
            } else if (statusNorm === 'Stored') {
                actionHtml = `
                    <button data-action="continue" data-id="${o.id}" type="button" aria-label="Continue stored order ${o.id}">Continue</button>
                    <button data-action="deliver" data-id="${o.id}" type="button" aria-label="Deliver order ${o.id}">Deliver</button>
                    <button data-action="fail" data-id="${o.id}" type="button" aria-label="Fail order ${o.id}">Fail</button>
                    <button data-action="lost" data-id="${o.id}" type="button" aria-label="Mark order ${o.id} as lost">Lost</button>`;
            } else if (statusNorm === 'In progress') {
                actionHtml = `
                    <button data-action="store" data-id="${o.id}" type="button" aria-label="Store order ${o.id} at a location">Store</button>
                    <button data-action="deliver" data-id="${o.id}" type="button" aria-label="Deliver order ${o.id}">Deliver</button>
                    <button data-action="fail" data-id="${o.id}" type="button" aria-label="Fail order ${o.id}">Fail</button>
                    <button data-action="lost" data-id="${o.id}" type="button" aria-label="Mark order ${o.id} as lost">Lost</button>`;
            } else {
                actionHtml = '<span>—</span>';
            }

            html += `<tr><td>${o.id}</td><td>${o.description}</td><td>${clientCell}</td><td>${destCell}</td><td>${statusNorm}</td><td>${actionHtml}</td></tr>`;
        }
        html += '</tbody></table>';
        orderListDiv.innerHTML = html;
        renderPagination(parsed.total);

        // Delegate button clicks
        orderListDiv.querySelectorAll('button[data-action]')?.forEach(btn => {
            btn.addEventListener('click', async (ev) => {
                const b = ev.currentTarget;
                const id = parseInt(b.getAttribute('data-id'), 10);
                const action = b.getAttribute('data-action');
                try {
                    if (action === 'take') {
                        await take_order(id);
                    } else if (action === 'deliver') {
                        await make_delivery(id);
                    } else if (action === 'store') {
                        const loc = prompt('Enter location id to store at:');
                        if (loc) await store_delivery(id, parseInt(loc, 10), null);
                    } else if (action === 'continue') {
                        await continue_delivery(id, null);
                    } else if (action === 'fail') {
                        const comment = prompt('Reason (optional):') || null;
                        await fail_delivery(id, comment);
                    } else if (action === 'lost') {
                        const comment = prompt('Note (optional):') || null;
                        await lose_delivery(id, comment);
                    }
                    // Re-render after state change
                    render();
                } catch (e) {
                    console.error('Action failed', action, id, e);
                }
            });
        });

        // Click client/destination to filter Orders list instead of navigating
        orderListDiv.querySelectorAll('a[data-show-location]')?.forEach(a => {
            a.addEventListener('click', (ev) => {
                ev.preventDefault();
                const id = parseInt(ev.currentTarget.getAttribute('data-show-location'), 10);
                const kind = ev.currentTarget.getAttribute('data-kind') || 'client';
                // Enforce physical locations only for filter selection
                const loc = byId.get(id);
                if (!loc || loc.is_physical !== true) {
                    alert('This location is not a physical facility and cannot be used as a filter.');
                    return;
                }
                if (kind === 'client') {
                    filterClient.value = String(id);
                    filterDestination.value = '';
                } else {
                    filterDestination.value = String(id);
                    filterClient.value = '';
                }
                // Ensure Orders tab is shown and update switch button
                const ordersTab = document.getElementById('tab-orders');
                if (ordersTab) ordersTab.click();
                updateSwitchButton();
                render();
            });
        });

        // Status summary
        const delivered = deliveries.filter(d => String(d.status).toUpperCase() === 'COMPLETE').length;
        const pending = deliveries.filter(d => String(d.status).toUpperCase() !== 'COMPLETE').length;
        statusDiv.textContent = `Pending: ${pending}, Delivered: ${delivered}`;
    }

    function renderPagination(total) {
        const pagDiv = document.getElementById('orders-pagination');
        if (!pagDiv) return;
        if (total <= perPage && currentPage === 1) {
            pagDiv.innerHTML = '';
            return;
        }
        const totalPages = Math.ceil(total / perPage);
        pagDiv.innerHTML = `
            <div class="pagination">
                <button id="prev-page" ${currentPage === 1 ? 'disabled' : ''}>Previous</button>
                <span>Page ${currentPage} of ${totalPages || 1}</span>
                <button id="next-page" ${currentPage >= totalPages ? 'disabled' : ''}>Next</button>
            </div>
        `;
        document.getElementById('prev-page')?.addEventListener('click', () => {
            if (currentPage > 1) {
                currentPage--;
                render();
            }
        });
        document.getElementById('next-page')?.addEventListener('click', () => {
            if (currentPage < totalPages) {
                currentPage++;
                render();
            }
        });
    }

    function renderDeliveries() {
        const delListDiv = document.getElementById('delivery-list');
        if (!delListDiv) return;
        const deliveries = getDeliveries();
        if (deliveries.length === 0) {
            delListDiv.innerHTML = '<p>No deliveries found.</p>';
            return;
        }

        const locationsList = (() => {
            try { return JSON.parse(get_locations()); } catch { return []; }
        })();
        const locById = new Map(locationsList.map(l => [l.id, l]));

        let html = '<table border="1" cellpadding="6" cellspacing="0" role="grid" aria-label="Deliveries list">';
        html += '<thead><tr><th>ID</th><th>Order #</th><th>Status</th><th>Location</th><th>Started</th><th>Ended</th><th>Comment</th></tr></thead><tbody>';
        for (const d of deliveries) {
            const locName = d.location_id ? (locById.get(d.location_id)?.name || d.location_id) : 'Carried';
            const statusLabel = String(d.status).replace(/_/g, ' ');
            const statusNorm = statusLabel.charAt(0).toUpperCase() + statusLabel.slice(1).toLowerCase();
            
            html += `<tr>
                <td>${d.id}</td>
                <td><a href="#" class="btn-link" data-goto-order="${d.order_number}">${d.order_number}</a></td>
                <td>${statusNorm}</td>
                <td>${locName}</td>
                <td><small>${d.started_at || '—'}</small></td>
                <td><small>${d.ended_at || '—'}</small></td>
                <td>${d.comment || '—'}</td>
            </tr>`;
        }
        html += '</tbody></table>';
        delListDiv.innerHTML = html;

        delListDiv.querySelectorAll('a[data-goto-order]').forEach(a => {
            a.addEventListener('click', (ev) => {
                ev.preventDefault();
                const num = ev.currentTarget.getAttribute('data-goto-order');
                filterSearch.value = num;
                document.getElementById('tab-orders').click();
                render();
            });
        });
    }

    function renderDashboard() {
        const el = document.getElementById('dashboard-summary');
        if (!el) return;
        try {
            const json = get_dashboard_summary();
            const s = JSON.parse(json || '{}');
            el.innerHTML = `
                <div role="group" aria-label="Completion summary">
                    <p><strong>East:</strong> ${s.east ?? 0}</p>
                    <p><strong>Central total:</strong> ${s.central_total ?? 0}</p>
                    <ul>
                        <li>A–E: ${s.central_ae ?? 0}</li>
                        <li>F–M: ${s.central_fm ?? 0}</li>
                        <li>N–W: ${s.central_nw ?? 0}</li>
                    </ul>
                    <p><strong>West:</strong> ${s.west ?? 0}</p>
                </div>
            `;
        } catch (e) {
            console.error('Failed to render dashboard summary', e);
            el.textContent = 'No summary available.';
        }
    }

    async function seedData() {
        try {
            console.log('Fetching demo data...');
            const [districts, locations, categories, orders] = await Promise.all([
                fetch('./data/districts.json').then(r => r.ok ? r.json() : Promise.reject(`Failed to load districts: ${r.status}`)),
                fetch('./data/locations.json').then(r => r.ok ? r.json() : Promise.reject(`Failed to load locations: ${r.status}`)),
                fetch('./data/delivery_categories.json').then(r => r.ok ? r.json() : Promise.reject(`Failed to load categories: ${r.status}`)),
                fetch('./data/orders.json').then(r => r.ok ? r.json() : Promise.reject(`Failed to load orders: ${r.status}`)),
            ]);
            console.log('Data fetched. Importing into local storage...');
            await import_districts(JSON.stringify(districts));
            await import_locations(JSON.stringify(locations));
            await import_delivery_categories(JSON.stringify(categories));
            await import_orders(JSON.stringify(orders));
            console.log('Import successful.');
            await set_schema_version('1');
            setupFilters();
            render();
        } catch (e) {
            console.error('Seeding failed', e);
            alert('Failed to load demo data. See console for details.');
        }
    }

    let googleAccessToken = null;
    const GOOGLE_CLIENT_ID = '461367032682-3vaqiqptnjdt6dqiidmd61ncp0ehvunn.apps.googleusercontent.com'; // User needs to replace this
    const SCOPES = "https://www.googleapis.com/auth/spreadsheets https://www.googleapis.com/auth/drive.file";

    function updateGoogleStatus(msg) {
        document.getElementById('google-status').textContent = msg;
    }

    async function initGoogle() {
        const btnLogin = document.getElementById('btn-google-login');
        const btnSheets = document.getElementById('btn-export-sheets');
        const btnImportSheets = document.getElementById('btn-import-sheets');
        const btnDrive = document.getElementById('btn-export-drive');
        const btnImportDrive = document.getElementById('btn-import-drive');

        if (!btnLogin) return;

        btnLogin.addEventListener('click', () => {
            const client = google.accounts.oauth2.initTokenClient({
                client_id: GOOGLE_CLIENT_ID,
                scope: SCOPES,
                callback: (response) => {
                    if (response.error !== undefined) {
                        throw (response);
                    }
                    googleAccessToken = response.access_token;
                    updateGoogleStatus('Logged in to Google');
                    btnSheets.disabled = false;
                    btnImportSheets.disabled = false;
                    btnDrive.disabled = false;
                    btnImportDrive.disabled = false;
                    btnLogin.textContent = 'Refresh Login';
                },
            });
            client.requestAccessToken({prompt: 'consent'});
        });

        btnSheets.addEventListener('click', exportToSheets);
        btnImportSheets.addEventListener('click', importFromSheets);
        btnDrive.addEventListener('click', saveToDrive);
        btnImportDrive.addEventListener('click', importFromDrive);
    }

    async function exportToSheets() {
        if (!googleAccessToken) return alert('Please login first');
        updateGoogleStatus('Exporting to Sheets...');
        try {
            // 1. Create a new spreadsheet
            const now = new Date();
            const dateStr = now.toISOString().replace(/[:.]/g, '-').slice(0, 19);
            const createResponse = await fetch('https://sheets.googleapis.com/v4/spreadsheets', {
                method: 'POST',
                headers: {
                    'Authorization': `Bearer ${googleAccessToken}`,
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    properties: { title: `Death Stranding Deliveries - ${dateStr}` }
                })
            });
            const spreadsheet = await createResponse.json();
            const spreadsheetId = spreadsheet.spreadsheetId;

            // 2. Prepare data
            const deliveries = JSON.parse(export_deliveries());
            const rows = [['ID', 'Order #', 'Status', 'Location ID', 'Started At', 'Ended At', 'Comment']];
            deliveries.forEach(d => {
                rows.push([d.id, d.order_number, d.status, d.location_id || '', d.started_at || '', d.ended_at || '', d.comment || '']);
            });

            // 3. Update values
            await fetch(`https://sheets.googleapis.com/v4/spreadsheets/${spreadsheetId}/values/Sheet1!A1:append?valueInputOption=USER_ENTERED`, {
                method: 'POST',
                headers: {
                    'Authorization': `Bearer ${googleAccessToken}`,
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ values: rows })
            });

            updateGoogleStatus(`Exported! Spreadsheet ID: ${spreadsheetId}`);
            window.open(`https://docs.google.com/spreadsheets/d/${spreadsheetId}`, '_blank');
        } catch (e) {
            console.error(e);
            updateGoogleStatus('Export to Sheets failed.');
        }
    }

    async function importFromSheets() {
        if (!googleAccessToken) return alert('Please login first');
        updateGoogleStatus('Listing spreadsheets from Drive...');
        try {
            // 1. List spreadsheets using Drive API
            const query = encodeURIComponent("mimeType='application/vnd.google-apps.spreadsheet' and trashed=false");
            const listResponse = await fetch(`https://www.googleapis.com/drive/v3/files?q=${query}&orderBy=modifiedTime desc&pageSize=10&fields=files(id,name,modifiedTime)`, {
                method: 'GET',
                headers: { 'Authorization': `Bearer ${googleAccessToken}` }
            });
            const data = await listResponse.json();
            const files = data.files || [];

            if (files.length === 0) {
                updateGoogleStatus('No spreadsheets found in your Drive.');
                return;
            }

            let promptMsg = "Select a spreadsheet to import (enter number):\n";
            files.forEach((f, i) => { promptMsg += `${i + 1}. ${f.name} (${new Date(f.modifiedTime || '').toLocaleString()})\n`; });

            const choice = prompt(promptMsg);
            if (choice === null) {
                updateGoogleStatus('Import cancelled.');
                return;
            }
            const idx = parseInt(choice) - 1;
            if (isNaN(idx) || !files[idx]) {
                alert('Invalid choice.');
                return;
            }

            const selectedFile = files[idx];
            updateGoogleStatus(`Fetching values from ${selectedFile.name}...`);

            // 2. Fetch sheet values
            // We assume Sheet1 or the first sheet, and columns ID, Order #, Status, Location ID, Started At, Ended At, Comment
            const spreadsheetId = selectedFile.id;
            const valuesResponse = await fetch(`https://sheets.googleapis.com/v4/spreadsheets/${spreadsheetId}/values/Sheet1!A2:G?valueRenderOption=UNFORMATTED_VALUE`, {
                method: 'GET',
                headers: { 'Authorization': `Bearer ${googleAccessToken}` }
            });

            if (!valuesResponse.ok) {
                throw new Error(`Failed to fetch values: ${valuesResponse.statusText}`);
            }

            const valuesData = await valuesResponse.json();
            const rows = valuesData.values || [];

            if (rows.length === 0) {
                updateGoogleStatus('No data found in the selected spreadsheet.');
                return;
            }

            // 3. Convert rows back to JSON format
            const deliveries = rows.map(row => ({
                id: parseInt(row[0]),
                order_number: parseInt(row[1]),
                status: row[2],
                location_id: row[3] ? parseInt(row[3]) : null,
                started_at: row[4] || null,
                ended_at: row[5] || null,
                comment: row[6] || null
            }));

            // 4. Import via JS
            updateGoogleStatus('Importing to local storage...');
            await import_deliveries(JSON.stringify(deliveries));

            updateGoogleStatus(`Successfully imported from ${selectedFile.name}!`);
            renderDeliveries();
        } catch (e) {
            console.error(e);
            updateGoogleStatus('Import from Sheets failed.');
        }
    }

    async function saveToDrive() {
        if (!googleAccessToken) return alert('Please login first');
        updateGoogleStatus('Saving to Drive...');
        try {
            const deliveriesJson = export_deliveries();
            const now = new Date();
            const dateStr = now.toISOString().replace(/[:.]/g, '-').slice(0, 19);
            const metadata = {
                name: `death_stranding_deliveries_${dateStr}.json`,
                mimeType: 'application/json'
            };

            const form = new FormData();
            form.append('metadata', new Blob([JSON.stringify(metadata)], { type: 'application/json' }));
            form.append('file', new Blob([deliveriesJson], { type: 'application/json' }));

            const response = await fetch('https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart', {
                method: 'POST',
                headers: { 'Authorization': `Bearer ${googleAccessToken}` },
                body: form
            });
            const file = await response.json();
            updateGoogleStatus(`Saved to Drive! File ID: ${file.id}`);
        } catch (e) {
            console.error(e);
            updateGoogleStatus('Save to Drive failed.');
        }
    }

    async function importFromDrive() {
        if (!googleAccessToken) return alert('Please login first');
        updateGoogleStatus('Listing files from Drive...');
        try {
            // 1. List files (only those the app can see, typically)
            // Filtering for JSON files that aren't in trash
            const query = encodeURIComponent("mimeType='application/json' and trashed=false");
            const listResponse = await fetch(`https://www.googleapis.com/drive/v3/files?q=${query}&orderBy=modifiedTime desc&pageSize=10&fields=files(id,name,modifiedTime)`, {
                method: 'GET',
                headers: { 'Authorization': `Bearer ${googleAccessToken}` }
            });
            const data = await listResponse.json();
            const files = data.files || [];

            if (files.length === 0) {
                updateGoogleStatus('No JSON files found in your Drive.');
                return;
            }

            // Simple prompt to pick a file (real app might use a custom modal)
            let promptMsg = "Select a file to import (enter number):\n";
            files.forEach((f, i) => { promptMsg += `${i + 1}. ${f.name} (${new Date(f.modifiedTime || '').toLocaleString()})\n`; });

            const choice = prompt(promptMsg);
            if (choice === null) {
                updateGoogleStatus('Import cancelled.');
                return;
            }
            const idx = parseInt(choice) - 1;
            if (isNaN(idx) || !files[idx]) {
                alert('Invalid choice.');
                return;
            }

            const selectedFile = files[idx];
            updateGoogleStatus(`Downloading ${selectedFile.name}...`);

            // 2. Download the file content
            const downloadResponse = await fetch(`https://www.googleapis.com/drive/v3/files/${selectedFile.id}?alt=media`, {
                method: 'GET',
                headers: { 'Authorization': `Bearer ${googleAccessToken}` }
            });

            if (!downloadResponse.ok) {
                throw new Error(`Failed to download file: ${downloadResponse.statusText}`);
            }

            const json = await downloadResponse.text();

            // 3. Import via JS
            updateGoogleStatus('Importing to local storage...');
            await import_deliveries(json);

            updateGoogleStatus(`Successfully imported from ${selectedFile.name}!`);
            renderDeliveries(); // Refresh the list if visible
        } catch (e) {
            console.error(e);
            updateGoogleStatus('Import from Drive failed.');
        }
    }

    function setupNav() {
        const tabs = ['dashboard', 'orders', 'deliveries', 'settings'];
        function show(panelId) {
            tabs.forEach(tt => {
                const panel = document.getElementById(`panel-${tt}`);
                const tab = document.getElementById(`tab-${tt}`);
                if (panel) panel.hidden = (tt !== panelId.replace('panel-', ''));
                if (tab) tab.setAttribute('aria-selected', (tt === panelId.replace('panel-', '')) ? 'true' : 'false');
            });
            if (panelId === 'panel-dashboard') renderDashboard();
            if (panelId === 'panel-orders') render();
            if (panelId === 'panel-deliveries') renderDeliveries();
        }

        tabs.forEach(t => {
            const btn = document.getElementById(`tab-${t}`);
            if (btn) {
                btn.addEventListener('click', () => show(`panel-${t}`));
            }
        });

        // Mobile filters toggle
        const toggle = document.getElementById('filter-toggle');
        const content = document.getElementById('filter-content');
        if (toggle && content) {
            toggle.addEventListener('click', () => {
                const expanded = toggle.getAttribute('aria-expanded') === 'true';
                toggle.setAttribute('aria-expanded', !expanded);
                content.classList.toggle('active');
            });
        }
        show('panel-orders');
    }

    function setupFilters() {
        // Populate selects from storage via JS getters (they return JSON strings)
        try {
            const districts = JSON.parse(get_districts());
            for (const d of districts) {
                const opt = document.createElement('option');
                opt.value = String(d.id);
                opt.textContent = d.name;
                filterDistrict.appendChild(opt);
            }
        } catch {
        }
        try {
            const locations = (JSON.parse(get_locations()) || []).filter(l => l && l.is_physical === true);
            for (const l of locations) {
                const opt1 = document.createElement('option');
                opt1.value = String(l.id);
                opt1.textContent = l.name;
                filterClient.appendChild(opt1.cloneNode(true));
                const opt2 = document.createElement('option');
                opt2.value = String(l.id);
                opt2.textContent = l.name;
                filterDestination.appendChild(opt2);
            }
        } catch {
        }
        try {
            const cats = JSON.parse(get_delivery_categories());
            for (const c of cats) {
                const opt = document.createElement('option');
                opt.value = String(c.id);
                opt.textContent = c.name;
                filterCategory.appendChild(opt);
            }
        } catch {
        }
        const onChange = () => {
            currentPage = 1; // Reset to first page on filter change
            render();
        };

        [filterDistrict, filterClient, filterDestination, filterCategory, filterSearch].forEach(el => {
            el.addEventListener('change', onChange);
        });
        filterSearch.addEventListener('keyup', onChange);

        document.querySelectorAll('input[name="status"], input[name="completion"]').forEach(el => {
            el.addEventListener('change', onChange);
        });

        filterDistrict.addEventListener('change', () => {
            // Sub-filter: Update Client and Destination based on District
            const districtId = filterDistrict.value;
            const locations = JSON.parse(get_locations());
            
            const clientOpts = filterClient.querySelectorAll('option');
            const destOpts = filterDestination.querySelectorAll('option');
            
            [clientOpts, destOpts].forEach(opts => {
                opts.forEach(opt => {
                    if (opt.value === "") {
                        opt.hidden = false;
                        return;
                    }
                    const loc = locations.find(l => String(l.id) === opt.value);
                    if (!districtId || (loc && String(loc.district_id) === districtId)) {
                        opt.hidden = false;
                    } else {
                        opt.hidden = true;
                    }
                });
            });
            // If current selection is now hidden, reset it
            if (filterClient.selectedOptions[0]?.hidden) filterClient.value = "";
            if (filterDestination.selectedOptions[0]?.hidden) filterDestination.value = "";
        });
    }

    // Initial run
    try {
        initialize();
    } catch (e) {
        console.error('initialize() failed', e);
    }

    const schema = (get_schema_version && get_schema_version()) || '';
    const hasOrders = getOrders().length > 0;
    const btnSeed = document.getElementById('btn-seed');
    if (btnSeed) btnSeed.addEventListener('click', seedData);
    if (!schema || !hasOrders) {
        const banner = document.getElementById('seed-banner');
        if (banner) banner.hidden = false;
    }

    setupNav();
    setupFilters();
    render();
    initGoogle();
}
