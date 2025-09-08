# Stranding App Panel User Manual

Last updated: 2025-09-07

This manual describes how to use the Death Stranding PoC app. It covers how to sign in, navigate the top‑navigation
resources, filter/search data, act on Orders and Deliveries, and move between related records and tabs.

Contents

- Sign in to the App panel
- Navigation model (Top navigation)
- Dashboard and widgets
- Orders
- Deliveries
- Filtering, sorting, search, and bulk actions
- Navigating between related records (links and tabs)
- Tips and conventions

## Sign in to the App panel

- App URL:
    - Example: https://pen-y-fan.github.io/death-stranding-poc/
- If you aren’t authenticated, the app shows the login screen. Use your standard application credentials.
  Authentication is handled via the app to use social login Auth (e.g. Google or GitHub).

## Navigation (Top navigation)

- The main navigation. You’ll see links as tabs across the top
  rather than a left sidebar.
- In this app exposes at least:
    - Dashboard
    - Orders
    - Deliveries
    - Locations
- Color palette (for context): primary = Indigo; success = Emerald; warning = Orange; danger = Rose; info = Blue; gray =
  Gray.

## Dashboard

- After login you land on the App dashboard.
- Default widgets for the dashboard:
    - Account panel information (profile and logout via the user menu)
- Additional App charts you may see on dashboard:
    - Orders doughnut chart
    - Complete orders central A to E Chart, 
    - Complete orders central F to M Chart, 
    - Complete orders central N to W Chart
    - Complete orders East Chart

## Orders

Path: /orders

Views available

- List: Table of orders with sortable/searchable columns, filters, and per‑row actions.
  - Act on orders as a player (take, store, continue, complete), and track personal progress.
    orders (taking, storeing, completing). The order is immutable, when a play acts on an order a delivery is created.
- View: Read‑only details for a single order, plus widgets (OrdersOverview) and related data.
- Edit: No option to edit orders.
- Create: No option to edit orders.

Key table columns (App panel)

- number (int) (sortable, searchable)
- name (wraps, searchable)
- max_likes (numeric, sortable, toggleable)
- weight (numeric, sortable, toggleable)
- client.name (wraps, sortable, toggleable)
- destination.name (wraps, sortable, toggleable)
- destination.district.name (sortable, hidden by default via column toggler)
- deliveryCategory.name (sortable, toggleable)
- userDeliveries.status (badge labeled “Deliveries”, searchable)

Filters available (above content)

- District: Filter by related districts that have locations.
- Client: Filter by client location (physical locations only).
- Destination: Filter by destination location (physical locations only).
- Delivery category: Filter by category.
- Delivery status: Select one status (In progress, Complete, Stored, Failed, Lost) to filter orders by the current
  user’s deliveries.
- Completion status: Choose Complete or Incomplete to include orders the current user has completed vs not yet
  completed.

Per‑row actions (contextual to the signed‑in user)

- Take on order: Creates an InProgress delivery for the current user if you don’t already have an active/Stored
  delivery for that order.
- Store delivery: For an InProgress delivery you hold, store it to a chosen location with optional comment.
- Continue delivery: For a Stored delivery you hold, resume it (status back to InProgress).
- Make delivery: Complete a delivery you hold (sets ended_at and marks COMPLETE, moving cargo to the client’s location).

Form fields (will be supplied to the app e.g JSON or Rust built in collection)

- number (int, required, unique, numeric) this is also a sequence.
- name (string, required)
- client (Location)
- destination (Location)
- delivery category (deliveryCategory - possibly an Enum or similar structured data)
- max_likes (float, required, numeric)
- weight (float, required, numeric, default 0)

Navigation from Orders to related records

- client.name and destination.name are clickable relation columns. Clicking either typically opens the related Location
  view (in the appropriate panel/resource).
- destination.district.name is visible (often via toggled column). To inspect the district, open the destination
  Location and then click its District link.

## Deliveries

Path: /deliveries

Views available

- List: Table of your deliveries (by default, query can be limited to user scope when implemented) with filters and
  actions.
- Create: Create a delivery (select order, location, status, optional times and comment).
- Edit: Update an existing delivery.
- Delete: Delete a delivery (e.g. if taken in error).

Key table columns

- order.number and order.name (click either to open the associated Order view)
- location.name
- started_at, ended_at (dates - in the user timezone)
- status (badge; values come from DeliveryStatus enum)
- comment (truncated, wrap)
- created_at / updated_at (toggleable, sortable)

Filters

- Status: Filter deliveries by status (e.g., In progress, Complete, Stored, Failed, Lost).

Row actions (buttons)

- Edit: opens edit form.
- Complete: Marks delivery COMPLETE, sets ended_at, and moves cargo to the order’s client location.
- Fail: Marks delivery FAILED and moves cargo to the order’s destination; optional comment recorded.
- Lost: Marks delivery LOST; optional comment recorded.
- Store: Switches to Stored at a chosen location (requires selecting a location and optional comment).
- Continue: Displayed for stored back to InProgress (optionally adding a comment). Some actions are only visible for
  compatible current statuses.

Create/Edit delivery form fields

- Order (searchable select: type order number or name)
- Location (searchable select - the current location of the order - can be a Location or carried)
- started_at (defaults to Europe/London now - should be set to the users current date time)
- ended_at (optional)
- status (radio from DeliveryStatus values)
- comment (optional)

## Filtering, sorting, search, and bulk actions

- Search: Use the search field above each table to search searchable columns.
- Sort: Click on sortable column headers; click again to toggle ascending/descending.
- Filters: Open the filters area above the content to combine filters (e.g., District + Delivery status).
- Column visibility: Use the column toggler to reveal or hide optional columns such as district or timestamps.
- Pagination: Use the paginator at the bottom to switch pages; adjust per‑page size if available.
- Bulk actions:
    - Orders: Accept and Complete (bulk) actions are available using checkboxes to select multiple rows.
      - Bulk Accept: Creates a delivery of each selected orders as InProgress (only if there are no existing deliveries in progress).
      - Bulk Complete: Marks selected orders.delivery as COMPLETE (even if no delivery exists, create a completed one, if one exists amend it to complete).
    - Deliveries: Bulk Delete is available.

## Navigating between related records (links and tabs)

- Linked columns: Clicking a relation value (e.g., client.name, destination.name, order.number on Deliveries) navigates
  to that record’s detail view.
- From Delivery → Order: order.number and order.name in Deliveries link to the Order view page.
- From Order → Locations/Districts: open the destination or client Location by clicking its name; from the Location you
  can proceed to its District where applicable.
- Breadcrumbs: Use the breadcrumb trail at the top to return to resource lists or parent pages.

## Tips and conventions

- Top navigation: The App panel uses tabs across the top instead of a sidebar, improving focus on operational tasks.
- Widgets: App widgets (charts/doughnuts) surface progress by district or region. Data is scoped to the signed‑in user
  for completion charts.
- Status badges: DeliveryStatus values render as colored badges across Orders and Deliveries.
- Validation: Forms validate required and unique fields; fix validation errors and resubmit.
- Timezone: Some actions set times with Europe/London timezone by default.

Support

- If you cannot sign in, verify you’re using the correct URL (/app) and that your account has access to the App panel.
- If select lists are empty (e.g., choosing a client/destination), ensure Locations and Delivery Categories exist and
  that relevant Locations are marked as physical when required.
