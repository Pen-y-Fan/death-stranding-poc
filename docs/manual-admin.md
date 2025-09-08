# Stranding Admin User Manual

Last updated: 2025-09-07

This manual describes how to use the Stranding admin (Filament 3) to manage Orders, Locations, Districts, and Delivery Categories. It focuses on how to sign in, navigate the main resource views, filter/search data, and move between related records.

Contents
- Sign in
- Dashboard overview
- Global navigation and search
- Orders
- Locations
- Districts
- Delivery Categories
- Filtering, sorting, search, and bulk actions
- Navigating between related records (tabs and links)
- Tips and conventions (Filament UI)

## Sign in
- Admin URL: /admin
  - Example: https://your-domain.test/admin
- You will be redirected to the Filament login page if not authenticated.
- Credentials: Use the account provided by your administrator. If your local app is fresh, ensure a user exists and you can authenticate with the default guard. The panel is configured to use Filament’s default authentication middleware.

## Dashboard overview
- After login you land on the Filament dashboard.
- Default widgets:
  - Account widget (top-right dropdown for profile/logout)
  - Filament Info widget
- The panel uses an amber primary color theme.
- Depending on permissions and configuration, you will see the navigation groups in the left sidebar. Common groups:
  - Orders
  - Settings → Locations, Districts, Delivery Categories

If custom widgets are added (e.g. charts), they will appear here. The code references an OrdersOverview widget for Orders.

## Global navigation and search
- Left sidebar: Access the main resources
  - Orders
  - Settings
    - Locations
    - Districts
    - Delivery Categories
- Top search (if enabled in the panel): Use it to find records across resources. Within a specific resource list, use that table’s Search input for column-aware searching.

## Orders
Path: Admin → Orders

Views available
- List: Table of orders with sortable, searchable columns.
- Create: Add a new order (number, name, client, destination, delivery category, max likes, weight).
- View: Read-only view of a single order with widgets such as OrdersOverview.
- Edit: Modify an existing order.

Key table columns
- number (sortable, searchable)
- name (wraps, searchable)
- destination.district.name (sortable)
- client.name (wraps, sortable)
- destination.name (wraps, sortable)
- deliveryCategory.name (wraps, sortable, can be toggled on/off in the column picker)
- max_likes (numeric, sortable, toggleable)
- weight (numeric, sortable, toggleable)
- deliveries.status (badge, searchable)
- created_at / updated_at (toggleable, sortable)

Filters available
- District (Select by related Districts that have Locations)
- Client (Select by Location where is_physical = true)
- Destination (Select by Location where is_physical = true)
- Delivery category
- Status checkboxes (multiple may be applied): In progress, Failed, Complete, Stashed, Lost. These filter by related Deliveries’ status.

Create/Edit form fields
- number (required, unique, numeric)
- name (required)
- client (Select from Locations via the client relationship)
- destination (Select from Locations via the destination relationship)
- delivery category (Select from Delivery Categories)
- max_likes (required, numeric)
- weight (required, numeric, default 0)

Navigating from an Order to related records
- Client and Destination columns are linked via relationships. Clicking a client/destination name typically opens the related Location’s view.
- Destination district is shown as destination.district.name. You can open the destination, then navigate to its District from the Location view.

## Locations
Path: Admin → Settings → Locations

Views available
- List: Table of locations.
- Create, View, Edit

Key table columns
- name (searchable)
- district.name (sortable)
- is_physical (boolean icon)
- client_orders_count (badge)
- destination_orders_count (badge)
- deliveries_count (badge)
- complete_orders_count (badge, green)
- created_at / updated_at (toggleable, sortable)

Filters available
- District (Select based on districts that have locations)

Create/Edit form fields
- name (required, unique)
- district (Select)
- is_physical (Toggle)
- Metadata placeholders: Created at / Last modified at (visible on existing records)

Related resources (tabs on Location view)
- Deliveries: Shows deliveries associated with this location.
- Complete Orders: Shows orders completed for this location.

Common navigation patterns from a Location
- Click the District name to open the District view.
- Use the relation tabs to see deliveries and completed orders without leaving the Location.

## Districts
Path: Admin → Settings → Districts

Views available
- List, Create, View, Edit

Key table columns
- name (searchable)
- locations_count (badge)
- created_at / updated_at (toggleable, sortable)

Related resources (tabs on District view)
- Locations: Manage or inspect locations belonging to this district.

Navigation
- From a District’s Locations tab, click a Location to drill down, then use that Location’s tabs (Deliveries, Complete Orders).

## Delivery Categories
Path: Admin → Settings → Delivery Categories

Views available
- List, Create, View, Edit

Key table columns
- name (searchable)
- orders_count (badge)
- created_at / updated_at (toggleable, sortable)

Related resources (tabs on Delivery Category view)
- Orders: Shows orders categorized under this delivery category.

Navigation
- From a Delivery Category’s Orders tab, click an Order to open its view. From there, you can navigate to the Client/Destination Locations.

## Filtering, sorting, search, and bulk actions
Applies across resources (patterns provided by Filament tables):
- Search: Use the search box above the table to search over searchable columns.
- Sort: Click column headers that indicate sortability. Click again to toggle asc/desc.
- Filters panel: Open the Filters sidebar/panel to apply select filters or checkboxes. Multiple filters can be combined.
- Column visibility: Use the column toggler to show/hide optional columns (e.g., deliveryCategory.name, created_at).
- Pagination: Use the paginator at the bottom to navigate pages. You can adjust per-page size if available.
- Row actions: Hover a row to reveal actions (View, Edit). Some resources also expose inline actions.
- Bulk actions: Select multiple rows with the checkboxes and use the Bulk actions dropdown (e.g., Delete on some resources).

## Navigating between related records
- Linked columns: Where a table column represents a relation (e.g., client.name or destination.name on Orders), clicking it typically takes you to the related record’s view page.
- Relation tabs: On a record’s View page, related resources appear as tabs. Examples:
  - Location → Deliveries, Complete Orders
  - District → Locations
  - Delivery Category → Orders
  You can browse, filter, and open related items without leaving the parent record.
- Breadcrumbs: Use the breadcrumb trail at the top to jump back to the resource list or parent pages.

## Tips and conventions (Filament UI)
- Keyboard: Use / to focus the search bar (if enabled by your browser/OS), and arrow keys to navigate table focus where supported.
- Resize and columns: On smaller screens, Filament wraps text and may stack actions into menus.
- Widgets: Some pages include cards/widgets. For Orders, a widget named OrdersOverview may appear providing summary metrics.
- Colors and badges: Statuses and counts appear as badges. Delivery status on Orders is a colored badge in the deliveries.status column.
- Safeguards: Forms validate required and unique fields. If you see validation errors, correct the input and resubmit.

Support
- If you encounter a permissions or sign-in issue, confirm you’re using the correct admin URL (/admin) and have a valid user/session.
- For data relationships or missing options (e.g., empty Client/Destination lists), ensure the related Locations and Delivery Categories exist, and that Locations marked as Client/Destination are physical when required by filters.
