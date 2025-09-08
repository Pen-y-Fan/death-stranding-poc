# Death Stranding PoC

I want to create a PoC for Death stranding

## Tech stack:

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

- IDE: Rust Rover by JetBrains
- Mac book m2 running IOS
- Brew (Package Manager) installed
- Rust installed: Rustup, cargo rustc are installed v1.89
