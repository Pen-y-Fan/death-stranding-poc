# Death Stranding PoC

Progress tracker for Death Stranding

## Purpose

The purpose of the death standing tracker is to track all deliveries with the aim to help make all 540 in the game!

Once complete this project will be available at https://pen-y-fan.github.io/death-stranding-poc.

### Dashboard

The dashboard displays graphs for every location in the game. This makes it easier to identify locations to target, such
as locations with a low delivery count. Later in the game its easy to identify locations with orders and complete
locations.

### Deliveries

A full table of all deliveries, including order details, status, and timestamps.

### Locations

When making multiple deliveries or collections from a location it can be useful to search by location and then switch
between deliveries to and from the location.

### Orders

This is the main screen to search, take and make deliveries by order.
- **Search**: Search by order number or name (always visible).
- **Filtering**: Combined filters for District, Client, Destination, Category, and Status.
- **Sub-filtering**: Selecting a District filters Client and Destination options.
- **Actions**: 'Start' an order (creates a delivery), 'Deliver' (completes it), 'Store', 'Fail', or mark as 'Lost'.
- **Pagination**: Automatic pagination (20 records per page) with Next/Previous navigation.
- **Status Labels**: Friendly labels like "In progress", "Complete", "Stored".
- **Buttons**: All links (e.g., Client/Destination) are styled as interactive buttons for better UX.

### Deliveries

A full list of your current and past deliveries.
- **Tracking**: View order numbers, status, current location, and timestamps.
- **Navigation**: Click an order number to jump back to that order in the main list.

## Roadmap

I want to expand the PoC for Death stranding, to include:

- A table of orders
- A table of deliveries

The only data structure with mutable data is the delivery, which is related to an order, and the order is related to a
client and destination. The delivery data should be persisted to local storage.

## Tech stack

- Rust compile to WASM
- Able to run locally with web browser server
- Able to deploy to GitHub page

## Requirements

- Table displaying five orders (static content - possible JSON or similar)
- When the web app is started local storage is checked and initialised )more details late)
- Button to take an order for delivery
- When the button is chosen a delivery is created
- The button to tank and order is hidden
- A button to deliver and order displays
- The current state of deliveries is saved to local storage
- When the web app is restarted it reads the deliveries from storage
- When the project is pushed to GitHub main (after a PR), a GutHub action will build the project and be available as a
  GitHub Page (static page with WASM and local storage for persistence)

## Local dev environment

- IDE: RustRover by JetBrains
- macOS with Homebrew
- Rust toolchain installed (rustup, cargo)

## Quickstart (Web build and local run)

See docs/manual-user.md and docs/description.md for detailed usage and domain context.

1. Install prerequisites:
    - rustup target add wasm32-unknown-unknown
    - cargo install wasm-pack
    - Optional local server: cargo install basic-http-server
2. Build for web:
    - wasm-pack build --target web --release
3. Serve the project root (so ./index.html can find ./pkg):
    - basic-http-server .
    - then open http://localhost:4000

## Testing

- Run native tests:

```shell
cargo test
```

- Optional WASM tests (headless in browser):

```shell
rustup target add wasm32-unknown-unknown
wasm-pack test --headless --chrome
```

## Deployment (GitHub Pages)

- The GitHub Actions workflow builds via wasm-pack and publishes docs/ with pkg/ + index.html.
- Manual setup: in your repository Settings → Pages, set Source to GitHub Actions.

## WASM API (brief)

Exposed functions (when compiled for wasm32):
- initialize()
- import_districts(json), import_locations(json), import_delivery_categories(json), import_orders(json), import_deliveries(json)
- export_deliveries() -> String (JSON)
- get_districts() -> String, get_locations() -> String, get_location(id) -> String, get_district(id) -> String, get_delivery_categories() -> String, get_orders() -> String
- take_order(number), store_delivery(number, location_id, comment?), continue_delivery(number, comment?), make_delivery(number), fail_delivery(number, comment?), lose_delivery(number, comment?)
- bulk_accept(numbers[]), bulk_complete(numbers[])
- query_orders(filter_json, page, per_page, sort_key, sort_dir, search) -> String
- get_dashboard_summary() -> String

## Notes

- Data directory: data/ at the project root. These JSON files are loaded by tests and optionally by the web app at
  runtime.
- Deployment: The GitHub Actions workflow copies data/ into docs/ alongside pkg/ and index.html so the files are
  available at /data/... on GitHub Pages.
- index.html imports ./pkg/death_stranding_poc.js which is generated by wasm-pack.
- Deliveries are persisted to browser localStorage; native builds noop those code paths behind cfg(target_arch = "
  wasm32").
- Timestamps: stored in ISO 8601 UTC; UI should present in Europe/London by default (or user’s TZ).
- Pinned dependency versions: wasm-bindgen 0.2.92, web-sys 0.3.78, serde 1.0.210, serde_json 1.0.128. To upgrade: bump
  versions in Cargo.toml keeping wasm-bindgen/web-sys on compatible pairs, then run `cargo update` and validate with
  `wasm-pack build` and `cargo test`.

## Google Integration

This application allows you to export your data to **your own** personal Google Drive and Google Sheets. The developer does **not** have access to your private files. 

### Why you need your own Client ID
Because this is a static Proof-of-Concept hosted on GitHub Pages (and not a verified commercial application), each person hosting or heavily testing the app should provide their own **Google Client ID**. This Client ID is like a "doorway" that allows the application to ask Google for permission to save files to **whoever is currently logged in**.

To enable this, follow these detailed steps to set up your project in the [Google Cloud Console](https://console.cloud.google.com/):

### 1. Create a New Project
- Click on the project selector in the top-left and select **New Project**.
- Give it a name like `Death Stranding PoC` and click **Create**.

### 2. Enable APIs
- Navigate to **APIs & Services > Library**.
- Search for and **Enable** both:
  - **Google Sheets API**
  - **Google Drive API**

### 3. Configure OAuth Consent Screen
- Go to **APIs & Services > OAuth consent screen**.
- Select **External**. This is the correct choice for a personal project where you want any user with a Google Account to be able to use it. Click **Create**.
- **App Information**: Fill in the required fields:
  - **App name**: `Death Stranding PoC`
  - **User support email**: Your email address.
  - **Developer contact information**: Your email address.
- **Scopes**: Click **Add or Remove Scopes**. In the "Manually add scopes" box, enter the following two URLs (separated by a comma) and click **Add to table**:
  - `https://www.googleapis.com/auth/spreadsheets`
  - `https://www.googleapis.com/auth/drive.file`
- Scroll down and click **Update**, then **Save and Continue**.
- **Test Users**: Add your own Google email address as a test user. Click **Save and Continue**.

### 4. Create OAuth 2.0 Client ID
- Go to **APIs & Services > Credentials**.
- Click **Create Credentials > OAuth client ID**.
- Select **Web application** as the Application type.
- **Name**: `Death Stranding Web Client`.
- **Authorized JavaScript origins**: Click **Add URI** and enter:
  - `http://localhost:4000` (for local development)
  - `https://pen-y-fan.github.io` (for your GitHub Pages deployment)
- Click **Create**.
- Copy the **Client ID** from the popup.

### 5. Update index.html
- Open `index.html` in your editor.
- Locate the `GOOGLE_CLIENT_ID` constant around line 358 and replace the placeholder:
  ```javascript
  const GOOGLE_CLIENT_ID = 'YOUR_GOOGLE_CLIENT_ID.apps.googleusercontent.com';
  ```

### 6. Local Testing
- Ensure you are serving the page from `http://localhost:4000` as configured in the origins.
- Go to the **Settings** tab in the app and click **Login with Google**.
- **Crucial Note**: When you log in, you are granting the app permission to access **your** Google Drive and Sheets, not anyone else's.
- Since the app is in "Testing" mode, you may see a "Google hasn't verified this app" warning. Click **Advanced > Go to Death Stranding PoC (unsafe)** to proceed.
- Once logged in, the **Export to My Google Sheets** and **Save to My Google Drive** buttons will be enabled. 
- These actions create a new spreadsheet or file in **your** account and open them in a new tab.

## Contributing

This is a **personal project**. Contributions are **not** required. Anyone interested in developing this project are
welcome to fork or clone for your own use.

## Credits

- [Michael Pritchard \(AKA Pen-y-Fan\)](https://github.com/pen-y-fan) original project

## License

MIT License (MIT). Please see [License File](LICENSE.md) for more information.

The order data has been copied from
the [Death stranding wiki](https://deathstranding.fandom.com/wiki/Orders) and is available under
the [CC BY-SA](https://www.fandom.com/licensing) license.  
