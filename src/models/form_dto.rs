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
pub struct FormDto {
    /// The form key.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The context path of the process application. If the task (or the process definition) does not belong to a process application deployment or a process definition at all, this property is not set.
    #[serde(rename = "contextPath", skip_serializing_if = "Option::is_none")]
    pub context_path: Option<String>,
}

impl FormDto {
    pub fn new() -> FormDto {
        FormDto {
            key: None,
            context_path: None,
        }
    }
}

