use actix_web::{HttpResponse, http::header::ContentType, Responder, get};
use rumbo_logic::metrics::{DiskSpaceInfo};

#[get("api/metric")]
pub async fn get_metric() -> impl Responder {
    let some_metric:DiskSpaceInfo = DiskSpaceInfo {
        name: "Some disk".to_string(),
        total_amount: 1000,
        free_amount: 24
    };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&some_metric).unwrap())
}