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

#[derive(Deserialize, Debug)]
pub struct ForPeriodQueryParameters {
    skip: Option<u64>,
    top: Option<u64>,
    start_period: Option<chrono::DateTime<Utc>>,
    end_period: Option<chrono::DateTime<Utc>>,
    instance_id: i64,
}

#[get("api/metric")]
pub async fn get_all_metrics(
    app: web::Data<RumboApp>,
    query: web::Query<ForPeriodQueryParameters>,
) -> impl Responder {
    info!("Tring to get all metrics with filter=\n{:?}", &query);

    let skip = match query.skip {
        None => 0,
        Some(val) => val as i64,
    };
    let top = match query.top {
        None => DEFAULT_PAGE_SIZE,
        Some(val) => val as i64,
    };
    let start_period = match query.start_period {
        None => get_week_ago_utc(),
        Some(val) => val,
    };
    let end_period = match query.end_period {
        None => get_utc_now(),
        Some(val) => val,
    };

    let metric_service = &app.metrics_service;
    let result = metric_service
        .for_period(query.instance_id, start_period, end_period, skip, top)
        .await
        .unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&result).unwrap())
}

#[post("api/metric")]
pub async fn create_metric(
    app: web::Data<RumboApp>,
    metric: web::Json<NewMetric>,
) -> impl Responder {
    info!("Trying to create metric");

    let metric_service = &app.metrics_service;
    let result = metric_service.create(metric.0).await.unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&result).unwrap())
}

#[delete("api/metric/{id}")]
pub async fn delete_metric(app: web::Data<RumboApp>, path: web::Path<String>) -> impl Responder {
    info!("Tring to delete metric with id={}", &path);

    let metric_service = &app.metrics_service;

    let id = i64::from_str(&path).expect("Invalid ID, number was expected");

    metric_service.delete(id).await.unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(())
}

#[patch("api/metric")]
pub async fn update_metric(app: web::Data<RumboApp>, metric: web::Json<Metric>) -> impl Responder {
    info!("Trying to update metric with id={}", metric.id);

    let metric_service = &app.metrics_service;
    let result = metric_service.update(metric.0).await.unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&result).unwrap())
}

fn get_week_ago_utc() -> chrono::DateTime<Utc> {
    let week_days = chrono::Days::new(7);
    chrono::offset::Utc::now()
        .checked_sub_days(week_days)
        .unwrap()
}

fn get_utc_now() -> chrono::DateTime<Utc> {
    chrono::offset::Utc::now()
}
