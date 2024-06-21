use candid::{CandidType, Deserialize};
use matchit::{Match, Params as MatchitParams};
use serde::Serialize;
use serde_json::{json, Value};
use std::{collections::HashMap, str::FromStr};

/// HeaderField is the type of the header of the request.
#[derive(CandidType, Deserialize, Clone)]
pub struct HeaderField(String, String);

/// RawHttpRequest is the request type that is sent by the client.
/// It is a raw version of HttpRequest. It is compatible with the Candid type.
/// It is used in the 'http_request' and 'http_request_update' function of the canister and it is provided by the IC.
/// It is converted to HttpRequest before it is used in the handler.
#[derive(CandidType, Deserialize, Clone)]
pub struct RawHttpRequest {
    pub(crate) method: String,
    pub(crate) url: String,
    pub(crate) headers: Vec<HeaderField>,
    #[serde(with = "serde_bytes")]
    pub(crate) body: Vec<u8>,
}

/// RawHttpResponse is the response type that is sent back to the client.
/// It is a raw version of HttpResponse. It is compatible with the Candid type.
#[derive(CandidType, Deserialize)]
pub struct RawHttpResponse {
    pub(crate) status_code: u16,
    pub(crate) headers: HashMap<String, String>,
    #[serde(with = "serde_bytes")]
    pub(crate) body: Vec<u8>,
    pub(crate) upgrade: Option<bool>,
}

impl RawHttpResponse {
    /// Set the upgrade flag of the response.
    fn set_upgrade(&mut self, upgrade: bool) {
        self.upgrade = Some(upgrade);
    }

    /// Enrich the header of the response depending on the content the body.
    fn enrich_header(&mut self) {
        if let None = self.headers.get("Content-Type") {
            self.headers.insert(
                String::from("Content-Type"),
                String::from("application/json"),
            );
        }
        self.headers
            .insert(String::from("X-Powered-By"), String::from("Pluto"));
    }
}
