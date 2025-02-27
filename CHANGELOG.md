### Changelog
Version 0.1
- All endpoints that are available from lta datamall website
- Configuration using API

Version 0.2 **[ Breaking Changes ]**
- Changed all API to take in `&LTAClient` rather than using a global `LTAClient`

Version 0.2.1
- Updated dependencies to latest version as of `21 July 2019`

Version 0.2.2 **[ Broken get_bus_stops, yanked from crates.io ]**
- Updated `LTAClient::with_api_key` to create a LTAClient

Version 0.2.3
- Hotfix for broken `lta::bus::get_bus_stops` which will panic due to typo in serde rename

Version 0.3.0-async-preview-1 **[ Breaking Changes ]**
- Client trait, now has 2 clients, one with async capabilities
- Currently using `futures-preview = "0.3.0-alpha.17"` and `tokio = "0.1.22"` 

Version 0.3.0-async-preview-2 **[ Breaking Changes ]**
- Re-exports to ensure compatibility (reqwest)
- Removed `futures-preview = "0.3.0-alpha.17"`
- Examples for all API, with the exception of `async`

Version 0.3.0-async-preview-3 **[ Breaking Changes ]**
- Removed some re-exports to avoid confusion
- Removed `futures-preview = "0.3.0-alpha.17"`
- Removed `tokio` as dependency and make it dev-dependency
- Added `futures = "0.1.28"`

Version 0.3.0-async-preview-4 **[ Breaking Changes ]**
- Re-exports to ensure compatibility (chrono)
- APIs that broke `bus::get_bus_arrival`, `traffic::get_bike_parking`

Version 0.3.0-async-preview-5
- Changed internals of deserialisation