/*
 * Rekor
 *
 * Rekor is a cryptographically secure, immutable transparency log for signed software releases.
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SearchIndex {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "publicKey", skip_serializing_if = "Option::is_none")]
    pub public_key: Option<crate::rekor::models::search_index_public_key::SearchIndexPublicKey>,
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
}

impl SearchIndex {
    pub fn new() -> SearchIndex {
        SearchIndex {
            email: None,
            public_key: None,
            hash: None,
        }
    }
}
