# zoho-rs

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

let client_id = "YOUR_CLIENT_ID";
let client_secret = "YOUR_CLIENT_SECRET";
let refresh_token = "YOUR_REFRESH_TOKEN";

let client = ZohoClient::with_creds(
    None, // access token
    None, // api domain
    String::from(client_id),
    String::from(client_secret),
    String::from(refresh_token)
);
```

## Roadmap

- [ ] Gracefully handle errors thrown back from Zoho
