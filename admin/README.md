# Korea Inside Accommodation Admin v1.2

Internal browser tool for building and reviewing Korea Inside accommodation data.

## Purpose

Accommodation Admin v1.2 expands the original CSV Analyzer into an internal Accommodation Admin System with rule-based first-pass review support.

CSV is the input module, not the final product. The tool turns imported accommodation rows into Korea Inside accommodation records with source CSV data, HRP analysis, zone classification, automatic criteria suggestions, final admin criteria, administrator notes and review status.

The exported result can later support a public accommodation page, a structured hotel database or another internal data workflow.

All v1.2 processing happens locally in the browser.

## Files

- `accommodation-analyzer.html` - internal admin page structure
- `accommodation-analyzer.css` - isolated admin styling
- `accommodation-analyzer.js` - CSV parsing, HRP analysis, auto suggestions, detail review state and export
- `README.md` - usage and development notes

No public page, shared CSS file or shared JavaScript file is required.

## Admin Structure

The v1.2 tool has five main modules:

| Module | Purpose |
|---|---|
| Dashboard | Shows dataset status and the internal review workflow |
| Accommodation List | Displays analyzed rows as the working accommodation database |
| Accommodation Detail | Shows imported data, automatic judgment, suggested criteria and final admin values |
| CSV Import | Keeps the original CSV upload, mapping, HRP and zone analysis workflow |
| Export | Exports the reviewed dataset as JSON or CSV |

## How To Use

1. Open `admin/accommodation-analyzer.html` in a browser.
2. Go to `CSV Import`.
3. Drag and drop a CSV file or use `Choose CSV`.
4. Review automatic column mapping.
5. Adjust HRP or zone thresholds if needed.
6. Select `Analyze`.
7. Go to `Accommodation List`.
8. Select `Review` for a property.
9. Compare suggested Korea Inside criteria with the admin final values.
10. Adjust final values, Admin Note and Review Status.
11. Save the review.
12. Check `Export` > `Local draft` to confirm the latest local save time.
13. Export JSON or CSV.
14. Use `Clear local draft` only when the browser recovery copy is no longer needed.

## Expected CSV Columns

The tool attempts to auto-map similar column names.

Recommended columns:

| Field | Example column names |
|---|---|
| Hotel Name | `Hotel Name`, `Property Name`, `Name` |
| Type | `Type`, `Property Type`, `Accommodation Type` |
| Address | `Address`, `Location`, `Street Address` |
| Latitude | `Latitude`, `Lat` |
| Longitude | `Longitude`, `Lng`, `Lon` |
| Booking URL | `Booking URL`, `URL`, `Link` |
| Rating | `Rating`, `Score`, `Review Score` |
| Review Count | `Review Count`, `Reviews`, `Number of Reviews` |
| Price | `Price`, `Rate`, `Nightly Price`, `Total Price` |

Required fields for analysis:

- Hotel Name
- Latitude
- Longitude

Rows without valid latitude or longitude remain in the list and are marked as `Needs Coordinates`.

## HRP Default

The v1.2 default Hongdae Reference Point is calculated from:

| Reference point | Latitude | Longitude |
|---|---:|---:|
| Hongik Univ. Station Exit 9 | 37.55667 | 126.92361 |
| Hongdae Red Road Entrance | 37.55598 | 126.92295 |

The final HRP fields can be manually edited before analysis.

## Distance And Walking Time

- Distance uses the Haversine formula.
- Walking time uses 80 meters per minute.
- Walking time is rounded up to the nearest minute.

## Default Korea Inside Zones

| Zone | Walking time |
|---|---:|
| Core | 0 to 5 minutes |
| Walkable | More than 5 to 10 minutes |
| Extended | More than 10 to 15 minutes |
| Nearby | More than 15 minutes |

The administrator can adjust the three threshold values before analysis.

## Auto Suggestion Rules

v1.2 generates rule-based first-pass suggestions for each accommodation.

| Criterion | Rule |
|---|---|
| Luggage Convenience | 5 if walking time is 5 minutes or less, 4 if 10 or less, 3 if 15 or less, 2 if more than 15, blank if coordinates are missing |
| Station Access | 5 if walking time is 5 minutes or less, 4 if 10 or less, 3 if 15 or less, 2 if more than 15, blank if coordinates are missing |
| Airport Access | Core or Walkable = 4, Extended = 3, Nearby = 2 |
| Nightlife Access | Core = 5, Walkable = 4, Extended = 3, Nearby = 2 |
| Noise Risk | Core = 4, Walkable = 3, Extended = 2, Nearby = 2 |
| Family Friendly | Core = 3, Walkable = 4, Extended = 4, Nearby = 3 |
| Shopping Access | Core = 5, Walkable = 4, Extended = 3, Nearby = 2 |
| Hill Risk | No automatic judgment in v1.2. Manual check is required. |

Hill Risk remains blank by default. The automatic Admin Note includes:

- `Manual check required: hill/slope condition.`

## Accommodation Detail

The detail screen separates:

- CSV Imported Data
- Korea Inside Automatic Judgment
- Suggested criteria values
- Admin final criteria values
- Admin Note
- Review Status

Review status options:

- Needs Review
- Reviewed
- Approved
- Excluded

## Auto Admin Note

Each new imported row receives a draft note:

- `{Hotel Name} is classified as {Zone}, approximately {walkingMinutes} minutes from HRP.`
- `Manual checks required: hill condition, actual luggage route, noise environment.`
- `Manual check required: hill/slope condition.`

Rows without valid coordinates receive a note that walking time could not be calculated.

## Export

The tool exports reviewed data as:

- JSON
- CSV

JSON rows include:

- `rawImportedData`
- `autoSuggestions`
- `finalCriteria`
- `adminNote`
- `reviewStatus`

CSV exports include raw imported data as a JSON string column, automatic suggestion columns, final criteria columns, Admin Note and Review Status.

## Local Draft Saving

The tool saves a local browser draft after CSV analysis, HRP or threshold changes, and admin review saves.

The local draft includes:

- CSV analysis results
- Column mapping
- HRP and zone threshold settings
- Admin final criteria values
- Admin Note
- Review Status
- Last saved time

When `admin/accommodation-analyzer.html` is reopened or refreshed in the same browser, the draft is restored automatically. The restored data remains local to that browser and is not uploaded to a server.

Use `Clear local draft` in the Export module to remove the saved browser recovery copy. Clearing the draft does not immediately clear the current on-screen data.

## v1.2 Limitations

- No server storage.
- Browser refresh can restore the latest local draft in the same browser, but localStorage is not a database or cross-device storage.
- No database connection.
- No map preview.
- No geocoding. Latitude and longitude must exist in the CSV.
- No duplicate detection.
- No official district boundary validation.
- Hill Risk cannot be automatically judged in v1.2.
- Noise, luggage route and hill/slope conditions still require manual review.
- No automated Booking.com crawl.
- CSV parsing is designed for normal exported CSV files, not every spreadsheet edge case.

## Possible v1.3 Features

- Local file re-import of previous JSON exports.
- Map preview using imported coordinates.
- Manual route and slope review fields.
- Duplicate accommodation detection.
- District boundary tagging for Hongdae, Yeonnam, Hapjeong, Sangsu and Mangwon.
- Korea Inside scoring model for suitability by traveler type.
- Saved presets for different Seoul districts.
