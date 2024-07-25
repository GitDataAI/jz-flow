/*
 * jiaozifs API
 *
 * jiaozifs HTTP API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserRegisterInfo {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "password")]
    pub password: String,
    #[serde(rename = "email")]
    pub email: String,
}

impl UserRegisterInfo {
    pub fn new(name: String, password: String, email: String) -> UserRegisterInfo {
        UserRegisterInfo {
            name,
            password,
            email,
        }
    }
}
