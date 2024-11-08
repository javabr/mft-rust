#![allow(unused_imports, unused_qualifications, unused_extern_crates)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ssh2::Sftp;
use std::io::Result;
use std::io::{Bytes, Error};
use std::path::Path;
use std::{
    collections::HashMap,
    ops::Deref,
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConnectionType {
    Sftp(SftpConnection),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasicInfo {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "lastModifiedDate")]
    pub last_modified_date: chrono::DateTime<chrono::Utc>,

    #[serde(rename = "createdDate")]
    pub created_date: chrono::DateTime<chrono::Utc>,

    #[serde(rename = "createdBy")]
    pub created_by: User,

    #[serde(rename = "createdBy")]
    pub modified_by: User,
}

impl BasicInfo {
    pub fn new(
        id: uuid::Uuid,
        name: String,
        description: Option<String>,
        last_modified_date: chrono::DateTime<chrono::Utc>,
        created_date: chrono::DateTime<chrono::Utc>,
        created_by: User,
        modified_by: User,
    ) -> BasicInfo {
        BasicInfo {
            id: id,
            name: name,
            description: description,
            last_modified_date: last_modified_date,
            created_date: created_date,
            created_by: created_by,
            modified_by: modified_by,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SftpConnection {
    #[serde(rename = "hostname")]
    pub hostname: String,

    #[serde(rename = "port")]
    pub port: u16,

    #[serde(rename = "userName")]
    pub user_name: String,

    #[serde(rename = "userPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_password: Option<String>,

    #[serde(rename = "identityKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_key: Option<String>,

    #[serde(rename = "identityKeyPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_key_password: Option<String>,

    #[serde(rename = "connectionTimeoutInSeconds")]
    pub connection_timeout_in_seconds: u32,

    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,

    #[serde(rename = "filename")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    #[serde(rename = "fileMask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_mask: Option<String>,
}

impl SftpConnection {
    pub fn new_identity_file_based_instance(
        hostname: String,
        port: u16,
        user_name: String,
        identity_key: Option<String>,
        identity_key_password: Option<String>,
        connection_timeout_in_seconds: u32,
        path: Option<PathBuf>,
        filename: Option<String>,
        file_mask: Option<String>,
    ) -> SftpConnection {
        SftpConnection {
            hostname: hostname,
            port: port,
            user_name: user_name,
            user_password: None,
            identity_key: identity_key,
            identity_key_password: identity_key_password,
            connection_timeout_in_seconds: connection_timeout_in_seconds,
            path: path,
            filename: filename,
            file_mask: file_mask,
        }
    }
    pub fn new_user_password_based_instance(
        hostname: String,
        port: u16,
        user_name: String,
        user_password: Option<String>,
        connection_timeout_in_seconds: u32,
        path: Option<PathBuf>,
        filename: Option<String>,
        file_mask: Option<String>,
    ) -> SftpConnection {
        SftpConnection {
            hostname: hostname,
            port: port,
            user_name: user_name,
            user_password: user_password,
            identity_key: None,
            identity_key_password: None,
            connection_timeout_in_seconds: connection_timeout_in_seconds,
            path: path,
            filename: filename,
            file_mask: file_mask,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndpointDefinition {
    #[serde(rename = "BasicInfo")]
    pub basic_info: BasicInfo,

    #[serde(rename = "connection")]
    pub conenction: ConnectionType,
}

impl EndpointDefinition {
    pub fn new(basic_info: BasicInfo, conenction: ConnectionType) -> EndpointDefinition {
        EndpointDefinition {
            basic_info: basic_info,
            conenction: conenction,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceDefinition {
    pub endpoint: EndpointDefinition,
}

impl SourceDefinition {
    pub fn new(endpoint: EndpointDefinition) -> SourceDefinition {
        SourceDefinition { endpoint: endpoint }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetDefinition {
    pub endpoint: EndpointDefinition,
}

impl TargetDefinition {
    pub fn new(endpoint: EndpointDefinition) -> TargetDefinition {
        TargetDefinition { endpoint: endpoint }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransferJobCommand {
    #[serde(rename = "basicInfo")]
    pub basic_info: BasicInfo,

    #[serde(rename = "source")]
    pub source: SourceDefinition,

    #[serde(rename = "targets")]
    pub targets: Vec<TargetDefinition>,
    // add before job, afterJob, beforeFile, afterFile
}

impl TransferJobCommand {
    pub fn new(
        basic_info: BasicInfo,
        source: SourceDefinition,
        targets: Vec<TargetDefinition>,
    ) -> TransferJobCommand {
        TransferJobCommand {
            basic_info: basic_info,
            source: source,
            targets: targets,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "lastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<chrono::DateTime<chrono::Utc>>,
}

impl User {
    pub fn new(
        id: uuid::Uuid,
        name: String,
        email: Option<String>,
        last_modified_date: Option<chrono::DateTime<chrono::Utc>>,
    ) -> User {
        User {
            id: id,
            name: name,
            email: email,
            last_modified_date: last_modified_date,
        }
    }
}
