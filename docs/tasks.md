# Project Tasks Checklist

Last generated: 2025-09-08 13:23

Below is an ordered, actionable checklist derived from docs/description.md, docs/manual-user.md, and the current
codebase. Each item starts with a checkbox [ ] to track completion. Tasks cover architecture, build/deploy, data
modeling, WASM boundary, business logic, UI shell, and testing. Where relevant, required input data is identified (e.g.,
JSON files for Orders, Locations, etc.).

1. [x] Architecture and build pipeline
    1. [x] Switch CI/CD (GitHub Pages workflow) to wasm-pack to generate ./pkg JS glue expected by index.html. (Build
       ./pkg and deploy index.html + pkg/)
    2. [x] Ensure toolchain steps in CI: install wasm32-unknown-unknown target and install wasm-pack.
    3. [x] Adjust workflow to publish docs/ with index.html and pkg/ using actions/upload-pages-artifact +
       actions/deploy-pages.
    4. [x] Document local dev quickstart in README: wasm-pack build --target web --release; basic-http-server .

2. [x] Crate configuration and dependencies
    1. [x] Confirm [lib] crate-type includes ["cdylib", "rlib"].
    2. [x] Ensure web-sys features include Window and Storage (already present); add any new APIs explicitly when used.
    3. [x] Pin compatible versions for wasm-bindgen, web-sys, serde/serde_json; document upgrade path.

3. [x] Data model definition (align with manual-user.md and description.md)
    1. [x] Define core structs with serde support:
        - [x] District { id, name, region (East/Central/West) }
        - [x] Location { id, name, district_id, is_physical }
        - [x] DeliveryCategory { id, name }
        - [x] Order { number (unique, int), name, client_id (Location), destination_id (Location), delivery_category_id,
          max_likes (f32), weight (f32) }
        - [x] DeliveryStatus enum { InProgress, STORED, COMPLETE, FAILED, LOST }
        - [x] Delivery { id, order_number, status: DeliveryStatus, location_id, started_at, ended_at, comment, user_id }
    2. [x] Provide JSON schemas/examples for seed data (see Required data section below) (created in `docs/schemas` and
       `docs/schemas/examples`).
    3. [x] Validate constraints (e.g., Order.number unique; required fields) at import time.

4. [x] Data loading and persistence (WASM boundary)
    1. [x] Implement JSON import functions exposed via wasm-bindgen to load master data into localStorage:
       districts.json, locations.json, delivery_categories.json, orders.json.
    2. [x] Implement getters to read current state from localStorage with robust error handling.
    3. [x] Implement JSON export/backup for deliveries per user.
    4. [x] Add versioning key in storage to handle migrations (e.g., schema_version).

5. [x] Business logic for deliveries (user actions)
    1. [x] take_order(number): create InProgress delivery if the user has no active or stored delivery for that order.
    2. [x] store_delivery(number, location_id, comment?): set status to STORED with location.
    3. [x] continue_delivery(number, comment?): move STORED → InProgress.
    4. [x] make_delivery(number): set COMPLETE, ended_at=now, move cargo to client location.
    5. [x] fail_delivery(number, comment?): set FAILED and move cargo to destination.
    6. [x] lose_delivery(number, comment?): set LOST.
    7. [x] Bulk actions:
        - [x] bulk_accept(order_numbers[]): create InProgress deliveries where none in-progress exist.
        - [x] bulk_complete(order_numbers[]): if no delivery exists, create COMPLETE; if exists, set COMPLETE.
    8. [x] Enforce status transition rules and edge cases; record timestamps in Europe/London (or user tz when
       available).

6. [x] Filtering, sorting, and search (data services)
    1. [x] Implement pure-Rust filter functions for Orders by: District, Client, Destination, DeliveryCategory,
       DeliveryStatus (for current user), Completion status.
    2. [x] Implement sort/search helpers (by number, name, weight, max_likes) to keep JS layer thin.
    3. [x] Provide WASM-exposed functions to query paginated/filtered lists suitable for a simple UI.

7. [x] UI shell integration (index.html minimal enhancements)
    1. [x] Replace placeholder table with functions that call new WASM APIs for lists and actions.
    2. [x] Add simple top navigation tabs: Dashboard, Orders, Deliveries, Locations (static HTML/JS).
    3. [x] Wire up per-row actions: Take, Store (with prompt for location/comment), Continue, Make delivery, Fail, Lost.
    4. [x] Add basic filters UI to Orders: dropdowns for District/Client/Destination/Category, radio for Delivery Status
       and Completion.

8. [ ] Charts and dashboard (optional PoC scope)
    1. [ ] Provide WASM-exposed summary endpoints for counts by region and location groups (Central A–E/F–M/N–W; East).
    2. [ ] Render simple doughnut/progress charts in JS (or text summaries as PoC).

9. [ ] Navigation between related records
    1. [ ] Provide WASM data lookups for relation expansion (e.g., get_location(id), get_district(id)).
    2. [ ] Update UI to make client/destination clickable to open related info panel.

10. [ ] Validation and error handling
    1. [ ] Validate inputs on all WASM-exposed functions; return Result-like error strings across the boundary.
    2. [ ] Handle localStorage errors (quota, unavailable) gracefully.

11. [ ] Testing (native and wasm)
    1. [ ] Add native unit tests for pure logic (status transitions, filtering, sorting, validation).
    2. [ ] Add wasm-bindgen-test for WASM boundary where feasible (optional headless).
    3. [ ] Ensure existing native build remains working under cfg(target_arch) gating.
    4. [ ] Provide test fixtures (JSON) for Orders/Locations/etc.
    5. [ ] Add tests for bulk actions edge cases (e.g., duplicate accept, complete without existing delivery).

12. [ ] Deployment (GitHub Pages)
    1. [ ] Update .github/workflows/deploy.yml to wasm-pack build and publish docs/ with pkg/ and index.html.
    2. [ ] Add a manual deployment note in README for GitHub Pages settings.
    3. [ ] Post-deploy smoke checklist: load page, verify pkg/death_stranding_poc.js exists, actions function.

13. [ ] Required data (you can supply these as JSON files) (available in `data/*.json`)
    1. [x] districts.json: list of Districts with region names (East/Central/West).
    2. [x] locations.json: Facilities with district_id and is_physical flag.
    3. [x] delivery_categories.json: Category taxonomy.
    4. [x] orders.json: All 540 Orders with required fields (number, name, client_id, destination_id,
       delivery_category_id, max_likes, weight).
    5. [x] users.json (optional): If multi-user scoping is needed; otherwise infer a single current user.
    6. [x] deliveries.json (optional initial state): Seed for demo/testing.

14. [ ] Time and timezone handling
    1. [ ] Decide and implement Europe/London default timestamping on actions; allow override for tests.
    2. [ ] Store timestamps in ISO 8601 UTC with separate display-timezone transformation in UI.

15. [ ] Documentation updates
    1. [ ] Update README with build/run/test instructions (wasm-pack, local server).
    2. [ ] Add brief API docs for exposed WASM functions.
    3. [ ] Link to docs/manual-user.md and docs/description.md from README.

16. [ ] Performance and size (PoC-appropriate)
    1. [ ] Keep web-sys features minimal; audit features if adding new APIs.
    2. [ ] Release builds with wasm-opt (if available via wasm-pack) for smaller payloads.

17. [ ] Accessibility and UX basics (PoC scope)
    1. [ ] Ensure buttons have accessible labels; keyboard navigation for list actions.
    2. [ ] Provide minimal ARIA roles for tables and status summaries.
