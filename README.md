# Death Stranding PoC

Progress tracker for Death Stranding.

## Purpose

The purpose of the Death Stranding tracker is to track all deliveries with the aim to help make all 540 in the game!

Once complete, this project will be available at https://pen-y-fan.github.io/death-stranding-poc.

### Dashboard

The dashboard displays graphs for every location in the game. This makes it easier to identify locations to target, such as locations with a low delivery count. Later in the game, it's easy to identify locations with orders and complete locations.

### Orders

This is the main screen to search, take, and make deliveries by order.
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
- **Persistence**: All delivery data is saved to your browser's local storage.

### Locations

When making multiple deliveries or collections from a location, it can be useful to search by location and then switch between deliveries to and from the location.

## Tech Stack

- **Frontend**: Zero-dependency pure JavaScript (ES6+), HTML5, and CSS3.
- **State Management**: Browser `localStorage` for data persistence.
- **Testing**: Node.js built-in test runner (`node:test`).
- **Deployment**: GitHub Pages via GitHub Actions.

## Requirements

- Node.js (v20+) for running the test suite.
- A modern web browser with JavaScript enabled.
- (Optional) A simple HTTP server for local development (to handle ES module imports correctly).

## Local Development

Since this is a pure JavaScript project, no compilation or build step is required.

1.  **Serve the project root**:
    Use a simple HTTP server so that `index.html` can correctly import the ES module from `./js/`.
    -   Using Python: `python3 -m http.server 4000`
    -   Using Node: `npx http-server . -p 4000`
2.  **Open the app**:
    Navigate to `http://localhost:4000` in your browser.

## Testing

The core business logic is verified using a JavaScript test suite.

1.  **Run all tests**:
    ```bash
    npm test
    ```
2.  **Run specific tests**:
    ```bash
    node --test tests_js/filtering.test.js
    ```

## Deployment

The project is automatically deployed to GitHub Pages via GitHub Actions whenever changes are pushed to the `main` branch. The workflow ensures all tests pass before updating the live site.

## API Overview

The core logic is encapsulated in `js/death_stranding_poc.js` and exports the following functions:

- `init()`: Default export for module initialization.
- `import_orders(json)`, `import_deliveries(json)`, etc.: Functions to load initial data.
- `export_deliveries()`: Returns a JSON string of all tracked deliveries.
- `query_orders(filter, page, per_page, sort_key, sort_dir, search)`: Main function for the Orders screen.
- `take_order(number)`, `make_delivery(number)`, `store_delivery(number, location_id)`, etc.: Functions for delivery lifecycle management.
- `get_dashboard_summary()`: Aggregates data for the dashboard visualizations.

## Google Integration

This application allows you to export your data to **your own** personal Google Drive and Google Sheets. The developer does **not** have access to your private files. 

### Why you need your own Client ID
Because this is a static Proof-of-Concept hosted on GitHub Pages (and not a verified commercial application), each person hosting or heavily testing the app should provide their own **Google Client ID**. This Client ID allows the application to ask Google for permission to save files to **whoever is currently logged in**.

To enable this, follow these steps to set up your project in the [Google Cloud Console](https://console.cloud.google.com/):

1.  **Create a New Project**: Give it a name like `Death Stranding PoC`.
2.  **Enable APIs**: Enable both **Google Sheets API** and **Google Drive API**.
3.  **Configure OAuth Consent Screen**:
    -   Select **External**.
    -   Add scopes: `https://www.googleapis.com/auth/spreadsheets` and `https://www.googleapis.com/auth/drive.file`.
    -   Add your own email as a **Test User**.
4.  **Create OAuth 2.0 Client ID**:
    -   Type: **Web application**.
    -   Authorized JavaScript origins: `http://localhost:4000` and your GitHub Pages URL.
5.  **Update index.html**: Replace the `GOOGLE_CLIENT_ID` constant in `index.html` with your new Client ID.
6.  **Local Testing**: Login with Google in the **Settings** tab of the app. You may need to click "Advanced" to proceed if the app is unverified.

## Credits

- [Michael Pritchard (AKA Pen-y-Fan)](https://github.com/pen-y-fan) - Original Author.

## License

This project is licensed under the [MIT License](LICENSE.md).

Order data is sourced from the [Death Stranding Wiki](https://deathstranding.fandom.com/wiki/Orders) and is available under the [CC BY-SA](https://www.fandom.com/licensing) license.
