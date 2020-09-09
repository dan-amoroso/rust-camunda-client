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
pub struct TaskQueryDtoSorting {
    /// Sort the results lexicographically by a given criterion. Must be used in conjunction with the sortOrder parameter.
    #[serde(rename = "sortBy", skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<SortBy>,
    /// Sort the results in a given order. Values may be `asc` for ascending order or `desc` for descending order. Must be used in conjunction with the sortBy parameter.
    #[serde(rename = "sortOrder", skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortOrder>,
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<crate::models::SortTaskQueryParametersDto>,
}

impl TaskQueryDtoSorting {
    pub fn new() -> TaskQueryDtoSorting {
        TaskQueryDtoSorting {
            sort_by: None,
            sort_order: None,
            parameters: None,
        }
    }
}

/// Sort the results lexicographically by a given criterion. Must be used in conjunction with the sortOrder parameter.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SortBy {
    #[serde(rename = "instanceId")]
    InstanceId,
    #[serde(rename = "caseInstanceId")]
    CaseInstanceId,
    #[serde(rename = "dueDate")]
    DueDate,
    #[serde(rename = "executionId")]
    ExecutionId,
    #[serde(rename = "caseExecutionId")]
    CaseExecutionId,
    #[serde(rename = "assignee")]
    Assignee,
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "description")]
    Description,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "nameCaseInsensitive")]
    NameCaseInsensitive,
    #[serde(rename = "priority")]
    Priority,
    #[serde(rename = "processVariable")]
    ProcessVariable,
    #[serde(rename = "executionVariable")]
    ExecutionVariable,
    #[serde(rename = "taskVariable")]
    TaskVariable,
    #[serde(rename = "caseExecutionVariable")]
    CaseExecutionVariable,
    #[serde(rename = "caseInstanceVariable")]
    CaseInstanceVariable,
}
/// Sort the results in a given order. Values may be `asc` for ascending order or `desc` for descending order. Must be used in conjunction with the sortBy parameter.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SortOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
