# Project Tasks Checklist

Last generated: 2025-09-08 13:23

Below is an ordered, actionable checklist derived from docs/description.md, docs/manual-user.md, and the current
codebase. Each item starts with a checkbox [ ] to track completion. Tasks cover architecture, build/deploy, data
modeling, WASM boundary, business logic, UI shell, and testing. Where relevant, required input data is identified (e.g.,
JSON files for Orders, Locations, etc.).

13. [x] Build and deploy process (pure JS).
    1. [x] Switch CI/CD (GitHub Pages workflow) to publish the ./js and ./css directories. (Build
       ./js, ./css and deploy index.html + js/ + css/)
    2. [x] Removed Rust toolchain from CI (transitioned to pure JS).
    3. [x] Adjust workflow to publish docs/ with index.html, js/ and css/ using actions/upload-pages-artifact +
       actions/deploy-pages.
    4. [x] Document local dev quickstart in README: serve the root directory and access index.html.

18. [x] Project structure (pure JS)
    1. [x] Removed obsolete Cargo.toml and Cargo.lock.
    2. [x] Removed src/ and tests/ (Rust source).
    3. [x] Removed target/ directory.

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

4. [x] Data loading and persistence (Browser storage)
    1. [x] Implement JSON import functions to load master data into localStorage:
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

6. [x] Filtering, sorting, and search (Data services)
    1. [x] Implement JS filter functions for Orders by: District, Client, Destination, DeliveryCategory,
       DeliveryStatus (for current user), Completion status.
    2. [x] Implement sort/search helpers (by number, name, weight, max_likes).
    3. [x] Provide JS functions to query paginated/filtered lists.

7. [x] UI shell integration
    1. [x] Use JS module for lists and actions.
    2. [x] Add simple top navigation tabs: Dashboard, Orders, Deliveries, Locations (static HTML/JS).
    3. [x] Wire up per-row actions: Take, Store (with prompt for location/comment), Continue, Make delivery, Fail, Lost.
    4. [x] Add basic filters UI to Orders: dropdowns for District/Client/Destination/Category, radio for Delivery Status
       and Completion.

8. [x] Charts and dashboard
    1. [x] Provide JS summary endpoints for counts by region and location groups.
    2. [x] Render simple doughnut/progress charts in JS (or text summaries as PoC).

9. [x] Navigation between related records
    1. [x] Provide JS data lookups for relation expansion (e.g., get_location(id), get_district(id)).
    2. [x] Update UI to make client/destination clickable to filter by client/destination.
    3. [x] Locations: add button to swap client/destination filter.

10. [x] Validation and error handling
    1. [x] Validate inputs on all JS functions; return appropriate error messages.
    2. [x] Handle localStorage errors (quota, unavailable) gracefully.

11. [x] Testing
    1. [x] Ported Rust unit tests to JavaScript using `node:test`.
    2. [x] Implement localStorage mock for JS test environment.
    3. [x] Add JS tests for status transitions, filtering, sorting, and validation.
    4. [x] Update CI/CD to run JS tests using `npm test`.
    5. [x] Removed legacy Rust tests.

12. [x] Deployment (GitHub Pages)
    1. [x] Update .github/workflows/deploy.yml to publish docs/ with js/, css/ and index.html.
    2. [x] Add a manual deployment note in README for GitHub Pages settings.
    3. [x] Post-deploy smoke checklist: load page, verify js/death_stranding_poc.js exists, actions function.

13. [x] Required data (you can supply these as JSON files) (available in `data/*.json`)
    1. [x] districts.json: list of Districts with region names (East/Central/West).
    2. [x] locations.json: Facilities with district_id and is_physical flag.
    3. [x] delivery_categories.json: Category taxonomy.
    4. [x] orders.json: All 540 Orders with required fields (number, name, client_id, destination_id,
       delivery_category_id, max_likes, weight).
    5. [x] users.json (optional): If multi-user scoping is needed; otherwise infer a single current user.
    6. [x] deliveries.json (optional initial state): Seed for demo/testing.

14. [x] Time and timezone handling
    1. [x] Decide and implement Europe/London default timestamping on actions; allow override for tests.
    2. [x] Store timestamps in ISO 8601 UTC with separate display-timezone transformation in UI.

15. [x] Documentation updates
    1. [x] Update README with run/test instructions (local server).
    2. [x] Document JS module API.
    3. [x] Transition from Rust/WASM to pure JavaScript for browser compatibility.
    3. [x] Link to docs/manual-user.md and docs/description.md from README.

16. [x] Performance and size (PoC-appropriate)
    1. [x] Optimized JS payload size by removing unused dependencies.
    2. [x] Fast execution due to native JS browser optimization.

17. [x] Accessibility and UX basics (PoC scope)
    1. [x] Ensure buttons have accessible labels; keyboard navigation for list actions.
    2. [x] Provide minimal ARIA roles for tables and status summaries.
