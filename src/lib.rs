//! # zoho-crm
//!
//! Library to help interact with v2 of the Zoho CRM API.
//!
//! You can read more information about the Zoho API here:
//! [https://www.zoho.com/crm/developer/docs/api/oauth-overview.html](https://www.zoho.com/crm/developer/docs/api/oauth-overview.html)
//!
//! If you plan on converting response records to custom structs, I highly recommend using `serde`:
//!
//! ```toml
//! [dependencies]
//! serde = { version = "1.0", features = ["derive"] }
//! ```
//!
//! ### Example
//!
//! ```no_run
//! use serde::Deserialize; // optional
//! use zoho_crm::ZohoClient;
//!
//! let client_id = String::from("YOUR_CLIENT_ID");
//! let client_secret = String::from("YOUR_CLIENT_SECRET");
//! let refresh_token = String::from("YOUR_REFRESH_TOKEN");
//!
//! let mut client = ZohoClient::with_creds(
//!     None, // access token
//!     None, // api domain
//!     client_id,
//!     client_secret,
//!     refresh_token
//! );
//!
//! #[derive(Deserialize)]
//! struct Account {
//!     id: String,
//!     name: String,
//! }
//!
//! let _accounts = client.get::<Vec<Account>>("/crm/v2/Accounts").unwrap();
//! ```

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_urlencoded;

mod client_error;
mod client;
mod token_record;

pub use client::Client as ZohoClient;
pub use client::parse_params;
pub use client_error::ClientError as ZohoError;
pub use token_record::TokenRecord as ZohoToken;
