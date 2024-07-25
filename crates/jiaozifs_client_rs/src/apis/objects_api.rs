/*
 * jiaozifs API
 *
 * jiaozifs HTTP API
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use bytes::Bytes;
use futures_core;
use reqwest::{self, multipart::Part};
use serde::{Deserialize, Serialize};

/// struct for typed errors of method [`delete_object`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteObjectError {
    Status401(),
    Status403(),
    Status404(),
    Status420(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_files`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFilesError {
    Status401(),
    Status404(),
    Status420(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_object`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetObjectError {
    Status401(),
    Status404(),
    Status410(),
    Status416(),
    Status420(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`head_object`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HeadObjectError {
    Status401(),
    Status404(),
    Status410(),
    Status416(),
    Status420(),
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`upload_object`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadObjectError {
    Status400(),
    Status401(),
    Status409(),
    Status403(),
    Status404(),
    Status412(),
    Status420(),
    UnknownValue(serde_json::Value),
}

pub async fn delete_object(
    configuration: &configuration::Configuration,
    owner: &str,
    repository: &str,
    ref_name: &str,
    path: &str,
) -> Result<(), Error<DeleteObjectError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/object/{owner}/{repository}",
        local_var_configuration.base_path,
        owner = crate::apis::urlencode(owner),
        repository = crate::apis::urlencode(repository)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("refName", &ref_name.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("path", &path.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteObjectError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_files(
    configuration: &configuration::Configuration,
    owner: &str,
    repository: &str,
    ref_name: &str,
    r#type: models::RefType,
    pattern: Option<&str>,
) -> Result<Vec<String>, Error<GetFilesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/object/{owner}/{repository}/files",
        local_var_configuration.base_path,
        owner = crate::apis::urlencode(owner),
        repository = crate::apis::urlencode(repository)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("refName", &ref_name.to_string())]);
    if let Some(ref local_var_str) = pattern {
        local_var_req_builder =
            local_var_req_builder.query(&[("pattern", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("type", &r#type.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetFilesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_object(
    configuration: &configuration::Configuration,
    owner: &str,
    repository: &str,
    ref_name: &str,
    path: &str,
    r#type: models::RefType,
    range: Option<&str>,
) -> Result<impl futures_core::Stream<Item = reqwest::Result<Bytes>>, Error<GetObjectError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/object/{owner}/{repository}",
        local_var_configuration.base_path,
        owner = crate::apis::urlencode(owner),
        repository = crate::apis::urlencode(repository)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("refName", &ref_name.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("path", &path.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("type", &r#type.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = range {
        local_var_req_builder =
            local_var_req_builder.header("Range", local_var_param_value.to_string());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(local_var_resp.bytes_stream())
    } else {
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: "".to_string(),
            entity: None,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn head_object(
    configuration: &configuration::Configuration,
    owner: &str,
    repository: &str,
    ref_name: &str,
    path: &str,
    r#type: models::RefType,
    range: Option<&str>,
) -> Result<(), Error<HeadObjectError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/object/{owner}/{repository}",
        local_var_configuration.base_path,
        owner = crate::apis::urlencode(owner),
        repository = crate::apis::urlencode(repository)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::HEAD, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("refName", &ref_name.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("path", &path.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("type", &r#type.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = range {
        local_var_req_builder =
            local_var_req_builder.header("Range", local_var_param_value.to_string());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<HeadObjectError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn upload_object(
    configuration: &configuration::Configuration,
    owner: &str,
    repository: &str,
    ref_name: &str,
    path: &str,
    is_replace: Option<bool>,
    content: Option<Vec<u8>>,
) -> Result<models::ObjectStats, Error<UploadObjectError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/object/{owner}/{repository}",
        local_var_configuration.base_path,
        owner = crate::apis::urlencode(owner),
        repository = crate::apis::urlencode(repository)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("refName", &ref_name.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("path", &path.to_string())]);
    if let Some(ref local_var_str) = is_replace {
        local_var_req_builder =
            local_var_req_builder.query(&[("isReplace", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form = reqwest::multipart::Form::new();
    // TODO: support file upload for 'content' parameter
    if let Some(content) = content {
        local_var_form = local_var_form.part("content", Part::bytes(content));
    }
    local_var_req_builder = local_var_req_builder.multipart(local_var_form);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UploadObjectError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
