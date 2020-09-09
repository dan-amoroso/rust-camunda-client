/*
 * Camunda BPM REST API
 *
 * OpenApi Spec for Camunda BPM REST API.
 *
 * The version of the OpenAPI document: 7.13.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AtomLink {
    /// The relation of the link to the object that belogs to.
    #[serde(rename = "rel", skip_serializing_if = "Option::is_none")]
    pub rel: Option<String>,
    /// The url of the link.
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// The http method.
    #[serde(rename = "method", skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
}

impl AtomLink {
    pub fn new() -> AtomLink {
        AtomLink {
            rel: None,
            href: None,
            method: None,
        }
    }
}

