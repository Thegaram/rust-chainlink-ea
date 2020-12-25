use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    pub id: String,
    pub data: JobData,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobData {
    pub address: String,
    pub data_prefix: String,
    pub function_selector: String,
    pub result: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JobResult {
    #[serde(rename = "jobRunID")]
    pub id: String,
}
