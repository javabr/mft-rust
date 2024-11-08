#![allow(unused_imports, unused_qualifications, unused_extern_crates)]
extern crate chrono;
extern crate uuid;

use serde::ser::Serializer;

use models;
use std::collections::HashMap;
use swagger;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseObject {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "lastModifiedDate")]
    pub last_modified_date: chrono::DateTime<chrono::Utc>,

    #[serde(rename = "createdBy")]
    pub created_by: models::User,
}

impl BaseObject {
    pub fn new(
        id: uuid::Uuid,
        name: String,
        description: String,
        last_modified_date: chrono::DateTime<chrono::Utc>,
        created_by: models::User,
    ) -> BaseObject {
        BaseObject {
            id: id,
            name: name,
            description: description,
            last_modified_date: last_modified_date,
            created_by: created_by,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceDefinition {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "lastModifiedDate")]
    pub last_modified_date: chrono::DateTime<chrono::Utc>,

    #[serde(rename = "createdBy")]
    pub created_by: models::User,
}

impl SourceDefinition {
    pub fn new(
        id: uuid::Uuid,
        name: String,
        description: String,
        last_modified_date: chrono::DateTime<chrono::Utc>,
        created_by: models::User,
    ) -> SourceDefinition {
        SourceDefinition {
            id: id,
            name: name,
            description: description,
            last_modified_date: last_modified_date,
            created_by: created_by,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetDefinition {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "lastModifiedDate")]
    pub last_modified_date: chrono::DateTime<chrono::Utc>,

    #[serde(rename = "createdBy")]
    pub created_by: models::User,
}

impl TargetDefinition {
    pub fn new(
        id: uuid::Uuid,
        name: String,
        description: String,
        last_modified_date: chrono::DateTime<chrono::Utc>,
        created_by: models::User,
    ) -> TargetDefinition {
        TargetDefinition {
            id: id,
            name: name,
            description: description,
            last_modified_date: last_modified_date,
            created_by: created_by,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransferJobCommand {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "lastModifiedDate")]
    pub last_modified_date: chrono::DateTime<chrono::Utc>,

    #[serde(rename = "createdBy")]
    pub created_by: models::User,

    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<models::SourceDefinition>,

    #[serde(rename = "targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<TargetDefinition>>,
}

impl TransferJobCommand {
    pub fn new(
        id: uuid::Uuid,
        name: String,
        description: String,
        last_modified_date: chrono::DateTime<chrono::Utc>,
        created_by: models::User,
    ) -> TransferJobCommand {
        TransferJobCommand {
            id: id,
            name: name,
            description: description,
            last_modified_date: last_modified_date,
            created_by: created_by,
            source: None,
            targets: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "lastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<chrono::DateTime<chrono::Utc>>,
}

impl User {
    pub fn new() -> User {
        User {
            id: None,
            name: None,
            email: None,
            last_modified_date: None,
        }
    }
}
