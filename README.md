# zohoxide-crm

Library to help interact with v2 of the Zoho CRM API.

## Description & Examples

You can either create a client with a preset access token, or fetch a new one later on. This can be useful if you are keeping track of you access tokens in a database, for example. You will need an API client ID, secret, and refresh token.

You can read more information here:
https://www.zoho.com/crm/developer/docs/api/oauth-overview.html

To handle parsing response records, you will also need deserializable objects with `serde`:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
```

## Example

```rust
use serde::Deserialize;
use zohoxide_crm::ZohoClient;

let client_id = String::from("YOUR_CLIENT_ID");
let client_secret = String::from("YOUR_CLIENT_SECRET");
let refresh_token = String::from("YOUR_REFRESH_TOKEN");

let mut client = ZohoClient::with_creds(
    None, // access token
    None, // api domain
    client_id,
    client_secret,
    refresh_token
);

#[derive(Debug, Deserialize)]
struct Account {
    id: String,
    name: String,
}

let account = client.get::<Account>("Accounts", "ZOHO_ID_HERE").unwrap();
```
