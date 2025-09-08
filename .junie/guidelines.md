Project-specific development guidelines

Context
- This repository is a Rust -> WebAssembly (WASM) proof-of-concept with a very thin JS/HTML shell (index.html) and Rust business logic in src/lib.rs exposed via wasm-bindgen.
- Deployment target is GitHub Pages. The current index.html expects a wasm-bindgen/wasm-pack style JS wrapper at ./pkg/death_stranding_poc.js.

Build and configuration
1) Toolchain prerequisites
- rustup target add wasm32-unknown-unknown
- Install wasm-pack for bundling and generating the ./pkg JS shim expected by index.html:
  cargo install wasm-pack
- Optional: a static HTTP server for local dev (choose one):
  cargo install basic-http-server
  or brew install http-server (Node), or use python3 -m http.server

2) Crate type and dependencies
- For WebAssembly export, library should be built as a cdylib. If not already present, add the following to Cargo.toml:
  [lib]
  crate-type = ["cdylib", "rlib"]

- web-sys features are gated at compile time. src/lib.rs uses window()/local_storage()/Storage when built for wasm32. To avoid link errors in wasm builds, enable the necessary web-sys features in Cargo.toml (this repo does not currently set them):
  [dependencies.web-sys]
  version = "0.3.78"
  features = [
      "Window",
      "Storage",
  ]

3) Building for the web
- The simplest path that matches index.html’s import is wasm-pack:
  wasm-pack build --target web --release
  This generates ./pkg/{death_stranding_poc_bg.wasm, death_stranding_poc.js, ...} which index.html imports.

- If you prefer raw cargo builds, you’ll only get a .wasm artifact; you’ll also need to generate or handcraft the JS glue expected by index.html, or switch your index.html to load the .wasm manually. Given the current index.html, use wasm-pack.

4) Local run
- After wasm-pack build --target web --release, serve the project root (so ./index.html can find ./pkg):
  basic-http-server .
  # then visit http://127.0.0.1:4000

Testing
1) Test strategy
- Native tests should not pull in web-sys/browser APIs. src/lib.rs already guards browser-only code behind cfg(target_arch = "wasm32"); native cargo test works without a browser.
- For unit tests touching pure logic, keep them in src or tests and avoid wasm-specific dependencies. For wasm-boundary behavior, prefer headless browser tests (e.g., wasm-bindgen-test) run under wasm32.

2) Running native tests (what currently works)
- Commands:
  cargo test
- Notes:
  - The repo compiles natively thanks to cfg gating in src/lib.rs; web APIs are no-ops in native builds.

3) Adding a new native test
- Create tests/<name>.rs with standard #[test] functions, e.g.:
  // tests/example.rs
  #[test]
  fn arithmetic_works() {
      assert_eq!(2 + 2, 4);
  }
- Run: cargo test

4) Optional: wasm tests (if needed later)
- Add dependency:
  wasm-bindgen-test = "0.3"
- In tests/wasm.rs:
  use wasm_bindgen_test::*;
  wasm_bindgen_test_configure!(run_in_browser);
  #[wasm_bindgen_test]
  fn it_works_in_browser() {
      assert_eq!(1 + 1, 2);
  }
- Run:
  rustup target add wasm32-unknown-unknown
  cargo test --target wasm32-unknown-unknown --no-default-features
  Or use wasm-pack test --headless --chrome (recommended if browsers are available in CI).

Deployment (GitHub Pages)
- Current workflow (.github/workflows/deploy.yml) builds with cargo build --target wasm32-unknown-unknown and copies only the .wasm into docs/, but index.html imports ./pkg/death_stranding_poc.js which is not produced by cargo alone. Without the JS glue, the page will fail to load.
- Recommended adjustment:
  - Use wasm-pack to generate ./pkg and deploy index.html + pkg/ + any assets.
  Example steps inside the workflow job:
    - uses: actions/checkout@v4
    - uses: actions-rs/toolchain@v1
      with:
        profile: minimal
        toolchain: stable
        target: wasm32-unknown-unknown
        override: true
    - name: Install wasm-pack
      run: cargo install wasm-pack
    - name: Build (wasm-pack)
      run: wasm-pack build --target web --release
    - name: Prepare artifact
      run: |
        mkdir -p docs
        cp -R pkg docs/
        cp index.html docs/
    - name: Upload artifact
      uses: actions/upload-pages-artifact@v3
      with:
        path: docs
    - name: Deploy
      uses: actions/deploy-pages@v4

- Alternatively, switch index.html to raw .wasm loading and keep the cargo-only build, but that requires custom JS loader code; wasm-pack is simpler and standard.

Runtime/Debugging notes
- The Rust code exposes initialize, take_order, and deliver_order when compiled to wasm32. In native builds these are no-op stubs (guarded by cfg) so cargo test remains fast and hermetic.
- LocalStorage access is done via web-sys::window().local_storage(). Make sure web-sys features (Window, Storage) are enabled for wasm builds (see dependencies above), otherwise linker errors will occur.
- alert is provided via #[wasm_bindgen] extern "C" for wasm; in native builds a no-op stub is compiled instead.

Code style and structure
- Edition: 2024 (per Cargo.toml). Use rustfmt with default settings; no custom rustfmt.toml present.
- Keep browser-bound code behind cfg(target_arch = "wasm32"). Provide graceful no-op stubs for native testability.
- Prefer serde for (de)serialization at the WASM boundary; validate JSON in browser before persisting to localStorage.
- If you add new web-sys APIs, explicitly enable the corresponding features in Cargo.toml to avoid large transitive builds and feature-missing errors.

Quickstart summary
- Build for web: wasm-pack build --target web --release
- Serve locally: basic-http-server . (open http://127.0.0.1:4000)
- Run tests: cargo test
- Add a test: create tests/example.rs and run cargo test

Known gaps to resolve in repo (actionable):
- Enable web-sys features (Window, Storage) in Cargo.toml for wasm builds.
- Update the GitHub Actions workflow to produce and deploy ./pkg via wasm-pack, or change index.html to load raw .wasm.
