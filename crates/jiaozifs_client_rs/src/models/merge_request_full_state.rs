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
pub struct MergeRequestFullState {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "sequence")]
    pub sequence: i32,
    #[serde(rename = "target_branch")]
    pub target_branch: uuid::Uuid,
    #[serde(rename = "source_branch")]
    pub source_branch: uuid::Uuid,
    #[serde(rename = "source_repo_id")]
    pub source_repo_id: uuid::Uuid,
    #[serde(rename = "target_repo_id")]
    pub target_repo_id: uuid::Uuid,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "merge_status")]
    pub merge_status: i32,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "author_id")]
    pub author_id: uuid::Uuid,
    #[serde(rename = "changes")]
    pub changes: Vec<models::ChangePair>,
    #[serde(rename = "created_at")]
    pub created_at: i64,
    #[serde(rename = "updated_at")]
    pub updated_at: i64,
}

impl MergeRequestFullState {
    pub fn new(
        id: uuid::Uuid,
        sequence: i32,
        target_branch: uuid::Uuid,
        source_branch: uuid::Uuid,
        source_repo_id: uuid::Uuid,
        target_repo_id: uuid::Uuid,
        title: String,
        merge_status: i32,
        author_id: uuid::Uuid,
        changes: Vec<models::ChangePair>,
        created_at: i64,
        updated_at: i64,
    ) -> MergeRequestFullState {
        MergeRequestFullState {
            id,
            sequence,
            target_branch,
            source_branch,
            source_repo_id,
            target_repo_id,
            title,
            merge_status,
            description: None,
            author_id,
            changes,
            created_at,
            updated_at,
        }
    }
}
