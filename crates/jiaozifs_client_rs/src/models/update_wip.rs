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
pub struct UpdateWip {
    #[serde(rename = "base_commit", skip_serializing_if = "Option::is_none")]
    pub base_commit: Option<String>,
    #[serde(rename = "current_tree", skip_serializing_if = "Option::is_none")]
    pub current_tree: Option<String>,
}

impl UpdateWip {
    pub fn new() -> UpdateWip {
        UpdateWip {
            base_commit: None,
            current_tree: None,
        }
    }
}

