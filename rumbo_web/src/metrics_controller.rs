use super::prelude::*;

#[get("api/metric/{id}")]
pub async fn get_metric(app: web::Data<RumboApp>, path: web::Path<String>) -> impl Responder {
    info!("Tring to get a metric with id={} ...", &path);

    let metric_service = &app.metrics_service;

    let id = i64::from_str(&path).expect("Invalid ID, number was expected");

    let result = metric_service.get(id).await.unwrap();

    if let Some(metric) = result {
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&metric).unwrap());
    };

    HttpResponse::NotFound()
        .content_type(ContentType::plaintext())
        .body("Model wasn't found")
}

#[post("api/metric")]
pub async fn create_metric(app: web::Data<RumboApp>, metric: web::Json<Metric>) -> impl Responder {
    info!("Trying to create metric...");

    let metric_service = &app.metrics_service;
    let result = metric_service.create(metric.0).await.unwrap();

    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&result).unwrap());
}

#[delete("api/metric/{id}")]
pub async fn delete_metric(app: web::Data<RumboApp>, path: web::Path<String>) -> impl Responder {
    info!("Tring to delete metric with id={} ...", &path);

    let metric_service = &app.metrics_service;

    let id = i64::from_str(&path).expect("Invalid ID, number was expected");

    metric_service.delete(id).await.unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(())
}

#[patch("api/metric")]
pub async fn update_metric(app: web::Data<RumboApp>, metric: web::Json<Metric>) -> impl Responder {
    info!("Trying to update metric with id={} ...", metric.id);

    let metric_service = &app.metrics_service;
    let result = metric_service.update(metric.0).await.unwrap();

    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&result).unwrap());
}
