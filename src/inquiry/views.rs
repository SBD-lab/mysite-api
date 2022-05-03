use std::str::FromStr;

use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    get, post,
    web::{Data, Json, Query},
    Error, HttpResponse,
};
use mongodb::bson::{self, doc};
use mongodb::sync::Collection;

use super::{MysiteInquiry, MysiteInquiryRequestData, MysiteInquiryRequestParams};

#[get("/api/inquiry")]
pub async fn get_inquiry(
    inquiry_coll: Data<Collection<MysiteInquiry>>,
    params: Query<MysiteInquiryRequestParams>,
) -> Result<HttpResponse, Error> {
    match &params.id {
        Some(str_id) => {
            let res_id = bson::oid::ObjectId::from_str(str_id);
            match res_id {
                Ok(inquiry_id) => {
                    let filter = doc! {"_id": inquiry_id};
                    inquiry_coll
                        .find_one(filter, None)
                        .map(|mysite_inquiry| HttpResponse::Ok().json(mysite_inquiry))
                        .map_err(|inquiry_err| ErrorInternalServerError(inquiry_err))
                }
                Err(_) => {
                    let message = format!("Invalid Object ID specified...");
                    Err(ErrorBadRequest(message))
                }
            }
        }
        None => {
            let message = format!("This is mysite-api to get inquiries!");
            Ok(HttpResponse::Ok().body(message))
        }
    }
}

#[post("api/inquiry")]
pub async fn post_inquiry(
    inquiry_coll: Data<Collection<MysiteInquiry>>,
    inquiry_data: Json<MysiteInquiryRequestData>,
) -> Result<HttpResponse, Error> {
    let inquiry_id = None;
    let closed = false;
    let datetime_now = bson::DateTime::now();
    let create_datetime = &datetime_now;
    let update_datetime = &datetime_now;
    let title = inquiry_data.0.title;
    let detail = inquiry_data.0.detail;
    let email = inquiry_data.0.email;
    let replies = Vec::new();

    let new_inquiry = MysiteInquiry {
        inquiry_id,
        closed,
        create_datetime: *create_datetime,
        update_datetime: *update_datetime,
        title,
        detail,
        email,
        replies,
    };

    inquiry_coll
        .insert_one(new_inquiry, None)
        .map(|mysite_inquiry| HttpResponse::Ok().json(mysite_inquiry))
        .map_err(|inquiry_err| ErrorInternalServerError(inquiry_err))
}
