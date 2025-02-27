//! ## Async requests for lta-rs
//! Currently uses `futures-preview = "0.3.0-alpha.17"`
//! API calling convention is exactly the same as the sync version
//!
//! ## Example
//! ```rust
//!
//! // async prelude includes prelude from sync prelude
//! use lta::r#async::{
//!     prelude::*,
//!     lta_client::LTAClient,
//!     bus::get_arrival,
//!     traffic::get_erp_rates
//! };
//! use std::env::var;
//! use tokio::run;
//!
//! fn async_example(client: &LTAClient) -> impl Future<Item = (), Error = ()> {
//!     type Req = (Vec<ErpRate>, BusArrivalResp);
//!     let fut = get_erp_rates(client);
//!     let fut2 = get_arrival(client, 83139, Some("15"));
//!     fut.join(fut2)
//!         .map(|(a,b): Req| {
//!             println!("{:?}", a);
//!             println!("{:?}", b);
//!     })
//!     .map_err(|e| println!("Request failed ${:?}", e))
//! }
//!
//! fn multiple_async_requests() {
//!     let api_key = var("API_KEY").unwrap();
//!     let client = &LTAClient::with_api_key(api_key);
//!     let fut = async_example(client);
//!     run(fut);
//! }
//! ```

pub mod bus;
pub mod crowd;
pub mod lta_client;
pub mod taxi;
pub mod traffic;
pub mod train;

/// Necessary imports to use lts-rs. Prefer this over glob imports
pub mod prelude {
    pub use futures::Future;

    pub use crate::prelude::*;
}
