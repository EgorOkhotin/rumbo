use super::prelude::*;

#[get("api/instance/{id}")]
pub async fn get_instance(app: web::Data<RumboApp>, path: web::Path<String>) -> impl Responder {
    info!("Tring to get a instance with id={}", &path);

    let instances_service = &app.instances_service;

    let id = i64::from_str(&path).expect("Incorrect Id format, number was expected.");

    let result = instances_service.get(id).await.unwrap();

    if let Some(instance) = result {
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&instance).unwrap());
    };

    HttpResponse::NotFound()
        .content_type(ContentType::plaintext())
        .body("Model wasn't found")
}

#[derive(Deserialize, Debug)]
pub struct InstanceQueryParameters {
    skip: Option<u64>,
    top: Option<u64>,
}
#[get("api/instance")]
pub async fn get_all_instances(
    app: web::Data<RumboApp>,
    query: web::Query<InstanceQueryParameters>,
) -> impl Responder {
    info!("Tring to get all instances with filter={:?}", &query);

    let skip = match query.skip {
        None => 0,
        Some(val) => val as i64,
    };

    let top = match query.top {
        None => DEFAULT_PAGE_SIZE,
        Some(val) => val as i64,
    };

    let instances_service = &app.instances_service;

    let result = instances_service.with_page(skip, top).await.unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&result).unwrap())
}

#[post("api/instance")]
pub async fn create_instance(
    app: web::Data<RumboApp>,
    instance: web::Json<Instance>,
) -> impl Responder {
    info!("Trying to create new instance");

    let instances_service = &app.instances_service;
    let result = instances_service.create(&instance.0).await.unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&result).unwrap())
}

#[delete("api/instance/{id}")]
pub async fn delete_instance(app: web::Data<RumboApp>, path: web::Path<String>) -> impl Responder {
    info!("Tring to delete instance with id={}", &path);

    let instances_service = &app.instances_service;

    let id = i64::from_str(&path).expect("Invalid Id, number was expected");

    instances_service.delete(id).await.unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(())
}

#[patch("api/instance")]
pub async fn update_instance(
    app: web::Data<RumboApp>,
    instance: web::Json<Instance>,
) -> impl Responder {
    info!("Trying to update instance with id {}", &instance.id);

    let instances_service = &app.instances_service;
    let result = instances_service.update(&instance.0).await.unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&result).unwrap())
}
