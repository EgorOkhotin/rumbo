use actix_web::{delete, patch};
use actix_web::{get, http::header::ContentType, post, web, HttpResponse, Responder};
use rumbo_logic::metrics::*;
use rumbo_logic::*;
use log::info;

#[get("api/metric/{id}")]
pub async fn get_metric(app: web::Data<RumboApp>, path: web::Path<String>) -> impl Responder {
    info!("Tring to get a metric with id={}", &path);

    let metric_service = MetricService::new(&app);

    let id_argument = path.as_str();
    let id = ObjectId::parse_str(id_argument).unwrap();

    let result = metric_service.get(&id).await.unwrap();

    if let Some(metric) = result {
        return HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&metric).unwrap())
    };

    HttpResponse::NotFound().content_type(ContentType::plaintext()).body("Model wasn't found")
}

#[post("api/metric")]
pub async fn create_metric(app: web::Data<RumboApp>, metric: web::Json<Metric>) -> impl Responder {
    info!("Trying to create metric with id");

    let metric_service = MetricService::new(&app);
    let result = metric_service.create(&metric.0).await.unwrap();

    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&result).unwrap())
}

#[delete("api/metric/{id}")]
pub async fn delete_metric(app: web::Data<RumboApp>, path: web::Path<String>) -> impl Responder {
    info!("Tring to delete metric with id={}", &path);

    let metric_service = MetricService::new(&app);

    let id_argument = path.as_str();
    let id = ObjectId::parse_str(id_argument).unwrap();

    metric_service.delete(&id).await.unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(())
}

#[patch("api/metric")]
pub async fn update_metric(app: web::Data<RumboApp>, metric: web::Json<Metric>) -> impl Responder {
    info!("Trying to update metric");

    let metric_service = MetricService::new(&app);
    let result = metric_service.update(&metric.0).await.unwrap();

    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&result).unwrap())
}
