pub mod settingstests;
pub mod sftptests;

use secrecy::{ExposeSecret, Secret};
use serde_aux::field_attributes::deserialize_number_from_string;
use std::{
    convert::{TryFrom, TryInto},
    path::PathBuf,
};

#[derive(serde::Deserialize, Clone)]
pub struct TestSettings {
    pub sftp1: SftpSettings,
    pub sftp2: SftpSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct SftpSettings {
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub path: String,
}
