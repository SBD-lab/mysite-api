use mongodb::bson;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MysiteInquiry {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub inquiry_id: Option<bson::oid::ObjectId>,
    pub closed: bool,
    pub create_datetime: bson::DateTime,
    pub update_datetime: bson::DateTime,
    pub title: String,
    pub detail: String,
    pub email: String,
    pub replies: Vec<MysiteInquiryReply>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MysiteInquiryReply {
    pub datetime: bson::DateTime,
    pub detail: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MysiteInquiryRequestData {
    pub title: String,
    pub detail: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MysiteInquiryRequestParams {
    pub id: Option<String>,
}
