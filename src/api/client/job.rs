use crate::{
    core::db::{
        Job,
        JobUpdateInfo,
    },
    utils::StdIntoAnyhowResult,
};
use anyhow::{
    anyhow,
    Result,
};

use mongodb::bson::oid::ObjectId;
use reqwest::{
    Client,
    StatusCode,
    Url,
};

pub struct JobClient {
    pub(crate) client: Client,
    pub(crate) base_uri: Url,
}

impl JobClient {
    pub async fn create(&self, job: &Job) -> Result<Job> {
        let resp = self
            .client
            .post(self.base_uri.clone().join("job")?)
            .json(&job)
            .send()
            .await
            .anyhow()?;

        if !resp.status().is_success() {
            let code = resp.status();
            let err_msg = resp
                .bytes()
                .await
                .anyhow()
                .and_then(|body| String::from_utf8(body.into()).anyhow())?;
            return Err(anyhow!("create job {code} reason {err_msg}"));
        }

        resp.bytes()
            .await
            .anyhow()
            .and_then(|body| serde_json::from_slice(&body).anyhow())
            .anyhow()
    }

    pub async fn get(&self, job_id: &ObjectId) -> Result<Option<Job>> {
        let resp = self
            .client
            .get(
                self.base_uri
                    .clone()
                    .join("job")?
                    .join(job_id.to_hex().as_str())?,
            )
            .send()
            .await
            .anyhow()?;

        if resp.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        if !resp.status().is_success() {
            let code = resp.status();
            let err_msg = resp
                .bytes()
                .await
                .anyhow()
                .and_then(|body| String::from_utf8(body.into()).anyhow())?;
            return Err(anyhow!("get job {code} reason {err_msg}"));
        }

        resp.bytes()
            .await
            .anyhow()
            .and_then(|body| serde_json::from_slice(&body).anyhow())
            .anyhow()
    }

    pub async fn delete(&self, job_id: &ObjectId) -> Result<()> {
        let resp = self
            .client
            .delete(
                self.base_uri
                    .clone()
                    .join("job")?
                    .join(job_id.to_hex().as_str())?,
            )
            .send()
            .await
            .anyhow()?;

        if !resp.status().is_success() {
            let code = resp.status();
            let err_msg = resp
                .bytes()
                .await
                .anyhow()
                .and_then(|body| String::from_utf8(body.into()).anyhow())?;
            return Err(anyhow!("delete job {code} reason {err_msg}"));
        }

        Ok(())
    }

    pub async fn update(&self, job_id: &ObjectId, update_info: &JobUpdateInfo) -> Result<()> {
        let resp = self
            .client
            .post(
                self.base_uri
                    .clone()
                    .join("job")?
                    .join(job_id.to_hex().as_str())?,
            )
            .json(update_info)
            .send()
            .await
            .anyhow()?;

        if !resp.status().is_success() {
            let code = resp.status();
            let err_msg = resp
                .bytes()
                .await
                .anyhow()
                .and_then(|body| String::from_utf8(body.into()).anyhow())?;
            return Err(anyhow!("request job {code} reason {err_msg}"));
        }

        Ok(())
    }
}
