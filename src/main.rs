use actix_web::{web::Data, App, HttpServer};
use inquiry::MysiteInquiry;
use mongodb::sync::Client;
use std::net::Ipv4Addr;

mod inquiry;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Azure Functions Port
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match std::env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    // Cosmos DB (Mongo DB) Connection
    let conn_string = std::env::var("MONGODB_URL").expect("failed to get MONGODB_URL");
    let mysite_db_name = std::env::var("MONGODB_DATABASE").expect("failed to get MONGODB_DATABASE");
    let mysite_collection_name =
        std::env::var("MONGODB_COLLECTION").expect("failed to get MONGODB_COLLECTION");

    let mongo_client = Client::with_uri_str(&conn_string).expect("failed to create client");
    let mongo_db = mongo_client.database(&mysite_db_name);
    let inquiry_coll = mongo_db.collection::<MysiteInquiry>(&mysite_collection_name);

    // HTTP Server Binding
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(inquiry_coll.clone()))
            .service(inquiry::get_inquiry)
            .service(inquiry::post_inquiry)
    })
    .bind((Ipv4Addr::UNSPECIFIED, port))?
    .run()
    .await
}
