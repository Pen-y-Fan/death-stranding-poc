Project-specific development guidelines (JavaScript-native version)

Context
- This repository is a JavaScript-based proof-of-concept for the "Death Stranding" delivery mechanics.
- Originally a Rust -> WASM project, it has been fully ported to a pure JavaScript module (`js/death_stranding_poc.js`) for better browser compatibility and stability.
- Deployment target is GitHub Pages. The `index.html` imports the core logic from `./js/death_stranding_poc.js`.

Project Structure
- `index.html`: Main entry point and UI.
- `js/death_stranding_poc.js`: Core business logic, models, and state management (ES module).
- `css/styles.css`: Application styling.
- `tests_js/`: JavaScript test suite using Node.js's built-in test runner.
- `data/`: Sample data and configuration schemas.

Build and Configuration
1) Development Workflow
- No compilation step is required. Changes to `js/` or `css/` are immediately reflected upon browser refresh.
- Use a simple HTTP server to serve the project root (to handle ES module imports correctly):
  - `python3 -m http.server`
  - `npx http-server .`
  - `basic-http-server .` (if installed via Cargo)

2) Dependencies
- Frontend: Zero-dependency (pure JS/CSS/HTML).
- Development/Testing: Node.js (v20+) is required for running the test suite.
- `package.json` defines the `test` script.

Testing
1) Test Strategy
- The core business logic is tested in a Node.js environment.
- Browser-specific APIs (like `localStorage`) are mocked in `tests_js/setup.js`.
- Tests are located in `tests_js/*.test.js`.

2) Running Tests
- Use npm (recommended):
  `npm test`
- Or run directly via Node.js:
  `node --test tests_js/*.test.js`

3) Adding a New Test
- Create a new file in `tests_js/` with the `.test.js` suffix.
- Use the `node:test` and `node:assert` modules.
- Ensure `setup.js` is imported if browser mocks are needed.

Deployment (GitHub Pages)
- The project is deployed via GitHub Actions (`.github/workflows/deploy.yml`).
- The workflow:
  1. Sets up Node.js.
  2. Runs the JS test suite (`npm test`).
  3. Prepares a `docs/` artifact containing:
     - `index.html`
     - `js/` directory
     - `css/` directory
     - `data/` directory (if applicable)
  4. Deploys the artifact to GitHub Pages.

Code Style and Structure
- Use modern ES6+ JavaScript.
- Maintain the module-based architecture (exporting functions/classes from `js/death_stranding_poc.js`).
- Follow the existing naming conventions (e.g., snake_case for internal logic parameters where it matches the original Rust implementation, camelCase for UI-bound variables).
- Keep the UI logic in `index.html` and business logic in `js/death_stranding_poc.js` separate.

Runtime/Debugging Notes
- The module exports an `init` function (default export) that handles initial data loading and state setup.
- `localStorage` is used for persisting delivery orders and state between sessions.
- Check the browser console for detailed logs from the JS module.

Quickstart Summary
- Serve locally: `npx http-server .` (visit http://localhost:8080)
- Run tests: `npm test`
- Add a test: Create `tests_js/my_feature.test.js` and use `node:test`.
