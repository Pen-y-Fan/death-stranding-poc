# Project Tasks Checklist

Last generated: 2026-03-24 21:55

Below is an ordered, actionable checklist derived from docs/description.md, docs/manual-user.md, and the current
codebase. Each item starts with a checkbox [ ] to track completion. Tasks cover architecture, build/deploy, data
modeling, WASM boundary, business logic, UI shell, and testing. Where relevant, required input data is identified (e.g.,
JSON files for Orders, Locations, etc.).

1. [x] Build and deploy process (pure JS).
    1. [x] Switch CI/CD (GitHub Pages workflow) to publish the ./js and ./css directories. (Build
       ./js, ./css and deploy index.html + js/ + css/)
    2. [x] Removed Rust toolchain from CI (transitioned to pure JS).
    3. [x] Adjust workflow to publish docs/ with index.html, js/ and css/ using actions/upload-pages-artifact +
       actions/deploy-pages.
    4. [x] Document local dev quickstart in README: serve the root directory and access index.html.

2. [x] Project structure (pure JS)
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
    4. [x] Link to docs/manual-user.md and docs/description.md from README.

16. [x] Performance and size (PoC-appropriate)
    1. [x] Optimized JS payload size by removing unused dependencies.
    2. [x] Fast execution due to native JS browser optimization.

17. [x] Accessibility and UX basics (PoC scope)
    1. [x] Ensure buttons have accessible labels; keyboard navigation for list actions.
    2. [x] Provide minimal ARIA roles for tables and status summaries.

18. [x] Mobile and Tablet UX Improvements (User Feedback)
    1. [x] Implement card-based layout for mobile and tablet devices for both Orders and Deliveries.
    2. [x] Use emojis for labels in card view (Order: 🎁, client: ⤴️, destination: ⤵️).
    3. [x] Increase button padding and add margin-bottom for better touch targets on mobile/tablet.
    4. [x] Place action buttons in the bottom row of the card.
    5. [x] Implement a hamburger menu for the main navigation on mobile devices.

19. [x] Search and Filter Enhancements (User Feedback)
    1. [x] Convert search input to a number field with validation (range: min to max order number).
    2. [x] Ensure partial numeric searching is still supported.
    3. [x] Increase search text size and add an 'X' button to clear search.
    4. [x] Consolidate filters behind a filter icon/button on all device sizes (responsive filter panel).
    5. [x] Restore the client and destination swap feature (⇅ button).
    6. [x] Add a 'reset' link (red text) to clear search and reset all filters to default.

20. [x] Dashboard and Analytics (User Feedback & Future Features)
    1. [x] Update completed orders logic to count unique orders instead of total deliveries.
    2. [x] Implement graphs and charts on the dashboard as described in manuals:
        - [x] Orders doughnut chart.
        - [x] Complete orders charts by range (A-E, F-M, N-W) and region (East).

21. [x] Deliveries Page Enhancements (Future Features)
    1. [x] Add search functionality to the deliveries page.
    2. [x] Implement edit and delete actions for individual deliveries.
    3. [x] Apply card-based layout to the deliveries list (similar to orders).
    4. [x] Implement bulk delete for deliveries.

22. [x] Developer Experience and CI/CD (Developer Feedback)
    1. [x] Investigate and fix GitHub Pages deployment race condition in `.github/workflows/deploy.yml`.
    2. [x] Add a Pull Request workflow to run the test suite on every PR.
    3. [x] Implement a static version number display in the top-right corner of the app.
    4. [x] Create a Pull Request template with a "Version number bumped?" checklist.
    5. [x] Fix schema validation in `.github/workflows/deploy.yml` (move `environment` to job level).

23. [x] Administrative and Advanced Features (Alignment with Manuals)
    1. [x] Implement "Settings" navigation for managing Locations, Districts, and Delivery Categories.
    2. [x] Add bulk actions for Orders: "Bulk Accept" and "Bulk Complete".
    3. [x] Implement field visibility toggler for cards (previously column visibility for tables).

24. [x] UI/UX Refinement and Dashboard Overhaul (Review Feedback)
    1. [x] Order and Deliveries Page Enhancements:
        - [x] Match heights of search inputs, bulk buttons, and filter toggles (48px).
        - [x] Align filter checkboxes and labels on the same line.
        - [x] Update client/destination swap button: use ⇄ emoji and match height to select inputs (35px + 4px margin top).
        - [x] Implement focus management and auto-scroll when searching by full order number.
        - [x] Add keyboard shortcuts for card actions: (s) start/store, (d) deliver, (l) lost, (f) fail.
        - [x] Fix Deliveries page search style and 'X' button positioning.
        - [x] Correct 'Inprogress' status text to 'In progress'.
        - [x] Format Started and Ended dates as "dd mmmm yyyy hh:mm:ss".
    2. [x] Card Design and Button Layout Updates:
        - [x] Remove 🎁 emoji from cards and enlarge order number text.
        - [x] Remove forced uppercase from location names.
        - [x] Update card heading hierarchy (H3 to H2) for accessibility.
        - [x] Hide delivery IDs on the Deliveries page cards.
        - [x] Reorder card buttons: STORE, LOST, FAIL (3-wide) then DELIVER (full-width on mobile).
        - [x] Increase pagination limit from 20 to 30 items per page for both pages.
    3. [x] Dashboard Analytics Expansion:
        - [x] Simplify Overall Completion display: "Orders Completed: x / 540 (y%)".
        - [x] Add "Current deliveries" section with status counts (Complete, In progress, Stashed, Lost, Failed).
        - [x] Implement detailed location-based stats (From/To) with progress bars for Central Region.
        - [x] Rename "Western Region" to "Eastern Region" and implement location-based stats.
        - [x] Audit and fix heading level jumps (H1 -> H2 -> H3) for accessibility.

25. [x] UI/UX Refinement and Code Quality (Review 2 Feedback)
    1. [x] Navigation and Responsive Design:
        - [x] Fix mobile menu button ("Menu") visibility on desktop and tablet (should be mobile-only).
        - [x] Add missing viewport meta tag to `index.html` to fix mobile scaling issues.
        - [x] Add padding-bottom to Order search `.search-input-wrapper` and filter containers `.bulk-actions` in mobile view.
        - [x] Add padding-bottom to Deliveries search `.search-input-wrapper`in mobile view.
    2. [x] Dashboard Overhaul (Phase 2):
        - [x] Update Dashboard cards to show both "From" (⤴️) and "To" (⤵️) stats in the heading.
        - [x] Implement dual progress bars for each location (one for outgoing, one for incoming).
        - [x] Split the Central Region dashboard card into multiple cards (max 6 locations per card) to avoid scrolling.
        - [x] Fix Dashboard card alignment: ensure "Overall Completion" and "Current Deliveries" occupy the first row, and Central Region cards start on row 2 (on desktop).
    3. [x] CI/CD and Code Quality:
        - [x] Split `deploy.yml` into separate workflows: `deploy.yml` (main branch only) and `test.yml` (pull requests).
        - [x] Refactor `styles.css` to use shorthand `padding` properties where applicable (e.g., `#delivery-search`).
        - [x] Audit and clean up `js/death_stranding_poc.js`:
            - [x] Remove unused functions (e.g., `get_location`, `get_district`, `get_orders`, `initSync`).
            - [x] Fix unresolved variables (e.g., `number`, `order_id`, `modifiedTime`, `id`, `name`).
            - [x] Remove redundant `await` calls for non-promise types.
            - [x] Simplify redundant logic and fix unresolved method references.
            - [x] Move all inline styles from `index.html` and `js/death_stranding_poc.js` to `css/styles.css`.
            - [x] Replace inline style injections in JS with CSS classes.
        - [x] Clean up Google API/OAuth2 related unresolved references if not used.
        - [x] Fix hover state for settings navigation buttons (prevent text color matching background color).

Last generated: 2026-03-24 23:32
