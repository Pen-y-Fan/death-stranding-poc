# Feedback

## User feedback (UO/UX)

The table is challenging to use on mobile and tablet. Can we change the table to a card? Possibly use emoji for the text
e.g. Order: 🎁, client: ⤴️, destination: ⤵️ etc.

Make the buttons larger (more padding) so they are easier to press on mobile and tablet. Add them to the bottom row of
the card. Add a margin bottom to each button.

Search needs to be a number field or type, with validation between the smallest and the largest number in the order
list. Still allow other numbers for partial searching. Make the search text larger, add an X to clear the search.

On all device sizes, mobile tablet and desktop hide the filter behind a filter icon currently in the mobile.

Add back the destination and client swap feature. This was previously added, not sure why it was removed. e.g. if the
client is selected, add a swap button ⇅ to swap the client and destination.

Add a 'reset' link (normally red text), which will remove search and reset filters to a default state. This was task 9
in the `docs/tasks.md`

On mobile devices make the main menu a hamburger menu.

On teh dashboard, When counting the number of completed orders, this should be a total unique delivery number, as some
orders may be have multiple deliveries.

## Developer feedback

Possibly a glitch in GitHub, but the last build failed due to the GitHub pages build starting before the new build had
completed. Can you check the build script is correct?

Add a PR build to run the tests.

Add a static version number to the top right so that we know which version has been deployed. After each release, the
version number will need to be bumped every release. Add a PR template to remind the version number.
e.g. [ ] Has the version number been bumped?

## Future features

The order page is good, it will be better after the user feedback is complete. The other menus can now be developed.

See the [user manual](manual-user.md) for more details.

1. the dashboard needs graphs and charts
2. the delivery page needs a search
3. a delivery can be edited and deleted
4. the delivery table can be a card, similar to the order table > card

## Review (UI/UX)

### Order page

The search and filter link is currently justified bottom, but the order input, bulk buttons and filter are all different
heights, so it looks janky, can they all be the same height as the order input (48 px)?

Filters: Card fields, input (checkboxes) and labels are on different lines, can they be on the same line?

Client and destination swap button: can the emoji be changed to ⇄,
Can the button be the same height as the select input? (35 px with 4 px margin top)

When searching by an order number, once the full order number is entered, and return or tab is pressed, if the order is
found, can the focus be moved to the button: either start (if the status is not in progress) or deliver (if the status
is in progress)? If the card is not in view, scroll to the card.

If the keyboard is used can the focus be moved to the card button when keys are pressed, e.g. When not in progress: s
for start, or when in progress: d for deliver, l for lost, s for store and f for fail.

Cards:

The order emoji 🎁 doesn't look good on the card, can it be removed, and the order number text be larger?

Remove uppercase from the name.

Is the 'name' heading accessible? Looks like we jumped from H1 to H3. Can it be H2?

Mobile buttons:

The Start button is full card width, width is OK. When started, (Status: In progress) the

STORE, DELIVER and LOST buttons are three wide, then the next line FAIL is full width.

Can the Deliver button be the last one, which is full width?

E.g. STORE, LOST and FAIL buttons are three wide, then the next line DELIVER is full width.

Desktop buttons:

Also change the order of the buttons to: STORE, LOST and FAIL and DELIVER

Pagination

20 per page is an odd number of cards when 3 wide, can it be 30 per page?

### Deliveries page

The order search should be the same height and style as the order search on the order page. It looks smaller and the X
is outside the input.

Status: 'Inprogress' should have a space: 'In progress'.

Make sure the bulk delete button is the same height as the search input, after changing.

The order emoji 🎁 doesn't look good on the card, can it be removed, and the order number text be larger? (Same as
revised Order card)

No need to show the ID.

Started and Ended dates are not formatted correctly. They should be user-friendly date and time. E.g. dd mmmm yyyy hh:
mm:ss

The cards should be paginated so that the user can see 30 deliveries at a time.

## Dashboard

Overall Completion:

Unique Orders Completed: 67
[progress bar]
~~12% of total orders~~

Change to:
Orders Completed: 67 / 540 (12%)
[progress bar]

No need to say "Unique" here. The percentage can be shown in the completed text, so no need for the sub-text.

Add:
Current deliveries

Complete (n)
In progress (n)
Stashed (n)
Lost (n)
Failed (n)

Central Region

Originally the dashboard showed graphs, A to E was all the locations for Chiral Artist's Studio to Evo-devo Biologist.
It may be worth listing each location with the number of deliveries made and also the number of deliveries expected e.g.
Chiral Artist's Studio
From: 4 / 14 (28%)
[progress bar]
To: 3 / 15 (20%)
[progress bar]

Collector
From: x / y (%)
[progress bar]
To: x / y (%)
[progress bar]

....

Weather station
From: x / y (%)
[progress bar]
To: x / y (%)
[progress bar]

Where 'from' is the client delivery for that location and 'to' is the destination delivery for that location.

They should also be the unique orders made to/from that location.

Other Regions:

Change to Eastern Region. There are no orders for Western Region.

Make the card for all locations in the Eastern region, similar to the Central region above.

Headings:

Check if headings are accessible? It looks like we jumped from H1 to H3.

## Review2 (UI/UX)

### Nav Bar

The button "Manu" for the mobile menu is visible on lablet and desktop:

```html

<button id="nav-toggle" class="mobile-only" aria-label="Toggle navigation" aria-expanded="true">
    <span class="hamburger"></span> Menu
</button>
```

The hamburger menu displays correctly on mobile.

### Responsive design

On my mobile (Pixel 7) the mobile view doesn't display. I get the tablet view, the text is too small. Is there a reason?
When I review phones in developer tools they are all displaying the tablet view.

### Order Filter (mobile)

The order search and filter need padding bottom on mobile view

### Dashboard cards

Cards look good, but they only show the orders 'from'. The 'to' is not displayed.

Only one status bar is displayed, it should be displayed one for from and one for to.

To improve the layout, change the heading:

Was: Chiral Artist's Studio: 5 / 15
Now: Chiral Artist's Studio: (⤴️ 5 / 15 33% ⤵️ 4 / 20 20%)

### Central Region card

I like the fixed card length but not the scroll. The central region card has too many locations in one card.
Can we split it into three or four cards with a maximum of seven locations per card? (A - D) = Chiral Artist's Studio to
Doctor) (E - ...) starts with (Elder - ....)

## Developer

### Split deploy and test

Can we split the deploy.yml:

- keep it for deploying from main only, it will still re-run the tests.
- create a test.yml for testing, which is only run for pull requests.

### style.css

Properties may be safely replaced with 'padding' shorthand line 273 - 275

```css
#delivery-search {
    font-size: 1.2rem;
    padding: 0.75rem;
    padding-right: 2.5rem;
    margin-top: 0;
    height: 48px;
    width: 100%;
}
```

### death_stranding_poc.js

There are 65 problems reported by the IDE, some maybe informational; can these be reviewed, unused code removed?:

- Unused function get_location
- Unused function get_district
- Unused function get_orders
- Unused function initSync
- Local variable num is redundant
- Unused property callback
- 'throw' of exception caught locally
- 'throw' of exception caught locally
- 'if' statement can be simplified
- Unresolved variable number
- Unresolved variable number
- Unresolved variable number
- Unresolved variable number
- Unresolved variable order_id
- Unresolved variable order_id
- Redundant 'await' for a non-promise type
- Redundant 'await' for a non-promise type
- Redundant 'await' for a non-promise type
- Redundant 'await' for a non-promise type
- Redundant 'await' for a non-promise type
- Redundant 'await' for a non-promise type
- Unresolved function or method updateSwitchButton()
- Redundant 'await' for a non-promise type
- Redundant 'await' for a non-promise type
- Redundant 'await' for a non-promise type
- Redundant 'await' for a non-promise type
- Redundant 'await' for a non-promise type
- Unresolved variable or type google
- Unresolved variable accounts
- Unresolved variable oauth2
- Unresolved function or method initTokenClient()
- Unresolved variable access_token
- Unresolved function or method requestAccessToken()
- Unresolved variable spreadsheetId
- Duplicated code fragment (14 lines long)
- Unresolved variable modifiedTime
- Redundant 'await' for a non-promise type
- Duplicated code fragment (14 lines long)
- Unresolved variable modifiedTime
- Redundant 'await' for a non-promise type
- Argument type boolean is not assignable to parameter type string
- Argument type boolean is not assignable to parameter type string
- Unresolved variable id
- Unresolved variable name
- Unresolved variable id
- Unresolved variable name
- Promise returned from initGoogle is ignored

## User experience feedback

When buttons are clicked, the buttons should show the click event, it should be temporarily disabled. It should provide
clear and concise feedback to the user about the action being performed. This includes displaying loading indicators
while the button is processing (this is normally instant, it would be good to add a delay to the UI), and providing
confirmation messages once the action is complete.

The current action is instantaneous, e.g. on accepting an order, when the filter is 'none' is applied, the order
instantly disappears. It would be better to display a clear button press, then fade or slide out the order.

On the order page, it would be good to have current progress displayed under the Orders heading, this is already
displayed in the Overall completion card, on the dashboard, e.g. Orders Completed: 258 / 540 (48%). Do not display the
full card, only the text under the Order heading.

When viewing, then clicking the 'bulk' buttons, they should show the number of orders selected. It would be good to have
a confirmation message. e.g. 'Bulk accept (2)' -> click -> '+2 orders accepted' (UI delay) -> 'Bulk accept'

## Technical feedback

 `death-stranding-poc/js/death_stranding_poc.js` 11 problems (IDE Warnings)

- Unused function initSync :546
- Unused constant delListDiv :558
- Duplicate declaration :571
- Duplicate declaration :583
- Duplicate declaration :812
- Duplicate declaration :1066
- Duplicate declaration :1078
- Duplicate declaration :1336
- Unused property callback :1570
- 'throw' of exception caught locally: 1681
- 'throw' of exception caught locally :1788

