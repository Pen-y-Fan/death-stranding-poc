[2026-03-15 12:19] - Updated by Junie
{
    "TYPE": "correction",
    "CATEGORY": "Documentation clarity",
    "EXPECTATION": "They wanted step-by-step Google Cloud Console instructions that match the current UI, including exact credential type, required scopes for Sheets and Drive, and specific authorized origins/redirect URIs.",
    "NEW INSTRUCTION": "WHEN guiding Google API setup THEN list exact console clicks, scopes, and URIs"
}

[2026-03-15 12:32] - Updated by Junie
{
    "TYPE": "correction",
    "CATEGORY": "Google OAuth ownership",
    "EXPECTATION": "They want exports to go to each signed-in user's own Google Drive/Sheets, not the developer's account.",
    "NEW INSTRUCTION": "WHEN documenting or implementing Google integration THEN ensure per-user storage to the signed-in account"
}

[2026-03-17 21:27] - Updated by Junie
{
    "TYPE": "correction",
    "CATEGORY": "Import option",
    "EXPECTATION": "They expected a visible Import feature to bring data from Google Sheets/Drive into the app.",
    "NEW INSTRUCTION": "WHEN implementing Google integrations UI THEN include a clear Import from Sheets/Drive option"
}

[2026-03-17 21:32] - Updated by Junie
{
    "TYPE": "correction",
    "CATEGORY": "Test data cleanup",
    "EXPECTATION": "They want only valid deliveries kept; remove placeholder test orders (e.g., orders 1 and 2) from exports/imports.",
    "NEW INSTRUCTION": "WHEN importing or exporting deliveries THEN exclude test orders and non-existent IDs; only persist valid deliveries"
}

[2026-03-17 21:39] - Updated by Junie
{
    "TYPE": "correction",
    "CATEGORY": "Demo data seeding",
    "EXPECTATION": "They expected the 'Load demo data' action to import only base data (orders, locations, districts, categories) and not create any deliveries.",
    "NEW INSTRUCTION": "WHEN loading demo data THEN skip importing deliveries.json"
}

[2026-03-17 21:39] - Updated by Junie
{
    "TYPE": "correction",
    "CATEGORY": "Demo data seeding",
    "EXPECTATION": "They expected 'Load demo data' to import only base data (orders, locations, districts, categories) and not create any deliveries; the Deliveries list should be empty after seeding.",
    "NEW INSTRUCTION": "WHEN seeding via seedData() THEN remove deliveries import and clear existing deliveries"
}

[2026-03-17 21:45] - Updated by Junie
{
    "TYPE": "preference",
    "CATEGORY": "Import from Sheets",
    "EXPECTATION": "They want an explicit Import from Google Sheets option in addition to Drive import.",
    "NEW INSTRUCTION": "WHEN implementing Google integrations UI THEN include Import from Google Sheets button"
}

[2026-03-17 21:57] - Updated by Junie
{
    "TYPE": "correction",
    "CATEGORY": "Import/Export date display",
    "EXPECTATION": "They expected a valid date to appear in the import prompts; JSON exports should include a proper timestamp in the filename so dates parse correctly, and '(Invalid Date)' should not be shown.",
    "NEW INSTRUCTION": "WHEN exporting to JSON or Sheets THEN include an ISO 8601 timestamp in the filename"
}

[2026-03-17 22:31] - Updated by Junie
{
    "TYPE": "correction",
    "CATEGORY": "GitHub Pages deployment",
    "EXPECTATION": "They want the site to load without console errors: styles.css must be found, unsupported Permissions-Policy headers removed, and WASM should initialize successfully.",
    "NEW INSTRUCTION": "WHEN deploying to GitHub Pages project site THEN use relative asset paths and verify file presence"
}

[2026-03-17 22:55] - Updated by Junie
{
    "TYPE": "preference",
    "CATEGORY": "Git workflow",
    "EXPECTATION": "They want commit messages to follow a specific format: 40 chars subject, 2 sentences body, Conventional Commits, and gitmoji.",
    "NEW INSTRUCTION": "WHEN committing code THEN subject < 40 chars, body <= 2 sentences, use Conventional Commits and gitmoji"
}

