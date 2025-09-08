# Stranding: Purpose and How This App Helps You 100% Death Stranding

Last updated: 2025-09-07

## What game is this about?

Death Stranding features a delivery meta‑game at its core: you connect the United Cities of America by completing
standard and premium orders between facilities. For true completionists, the target is to complete all deliveries — 540
in total — spanning three regions:

- East Region
- Central Region
- West Region

While the in‑game UI shows per‑facility progress, it becomes difficult to answer cross‑cutting questions such as “Which
Central locations from A–E are still missing?” or “Which orders have I started, stashed, or finished as this user?”

## What does this app do?

This app is a focused progress tracker for Death Stranding deliveries. It models the game’s world and orders so you can
plan, execute, and verify completion across all 540 deliveries.

Key concepts mirrored from the game:

- Districts/Regions → District and Location models (facilities and hubs; Locations marked as physical where applicable)
- Orders → Order model (number, name, client, destination, category, weight, max likes)
- Deliveries (per user) → Delivery model with statuses: In progress, Stashed, Complete, Failed, Lost

With these models, the app provides:

- Two panels:
    - Admin (/admin): manage master data (Orders, Locations, Districts, Delivery Categories)
    - App (/app): act on deliveries as a player (take, stash, continue, complete), and track personal progress
- Rich filtering: filter Orders by district/region, client, destination, category, delivery status, and completion state
- Relation navigation: jump between Orders, Locations, and Districts to see context and progress per facility
- Charts and widgets: visualize completion by region or location groups (e.g., Central A–E, F–M, N–W; East), showing
  Delivered vs Incomplete for the signed‑in user
- Bulk actions: accept or complete multiple orders when applicable

## How it helps you get 100% completion

The app is designed to make the 540‑delivery goal manageable:

1) Plan by region

- Use the Orders list filters to scope to a region (via District) and then narrow to specific facilities (client or
  destination).
- Use the charts (e.g., “Central region A–E”) to quickly see where you are missing completions.

2) Execute efficiently

- From Orders, use per‑row actions to:
    - Take on order → creates your In progress delivery
    - Stash delivery → temporarily store cargo at a facility
    - Continue delivery → resume a stashed run
    - Make delivery → mark Complete and move cargo to the client location
- From Deliveries, you can also complete, fail, stash, continue, or mark lost with optional comments.

3) Verify and iterate

- Switch the Orders filter “Completion status” between Complete and Incomplete to confirm which orders you still need
  for 100%.
- Click into Locations to see counts of client/destination orders and “Orders completed” badges for an immediate
  facility‑level overview.
- Use District and Location relation tabs to traverse remaining work without losing context.

## Typical workflow to 100%

- Pick a region (East, Central, West), then open Orders and filter by that region’s district(s).
- Toggle Completion status to Incomplete to see what remains.
- Sort by number or name as you prefer, then start taking on orders.
- Use the App panel actions to progress each delivery (take → stash/continue as needed → make delivery).
- Watch the charts update as Delivered counts rise; drill into Locations where counts don’t match expectations.
- Repeat for each region until all 540 deliveries are Complete for your user.

## Tips

- Use column toggles to reveal district and timestamp columns when needed.
- When searching orders, try order number first, then partial name matches.
- Timezone default for actions is Europe/London; adjust dates manually in Deliveries if you’re logging historical runs.

If you’re new to the UI, see docs/manual.md (Admin) and docs/user-manual.md (App) for step‑by‑step navigation details.
Happy connecting!
