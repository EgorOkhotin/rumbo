use super::prelude::*;

#[get("api/instance/{id}")]
pub async fn get_instance(app: web::Data<RumboApp>, path: web::Path<String>) -> impl Responder {
    info!("Tring to get a instance with id={}", &path);

    let instances_service = &app.instances_service;

    let id = path.to_string();

    let result = instances_service.get(&id).await.unwrap();

    if let Some(instance) = result {
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&instance).unwrap());
    };

    HttpResponse::NotFound()
        .content_type(ContentType::plaintext())
        .body("Model wasn't found")
}

#[post("api/instance")]
pub async fn create_instance(app: web::Data<RumboApp>, instance: web::Json<Instance>) -> impl Responder {
    info!("Trying to create instance with id");

    let instances_service = &app.instances_service;
    let result = instances_service.create(&instance.0).await.unwrap();

    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&result).unwrap());
}

#[delete("api/instance/{id}")]
pub async fn delete_instance(app: web::Data<RumboApp>, path: web::Path<String>) -> impl Responder {
    info!("Tring to delete instance with id={}", &path);

    let instances_service = &app.instances_service;

    let id = path.as_str();

    instances_service.delete(id).await.unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(())
}

#[patch("api/instance")]
pub async fn update_instance(app: web::Data<RumboApp>, instance: web::Json<Instance>) -> impl Responder {
    info!("Trying to update instance");

    let instances_service = &app.instances_service;
    let result = instances_service.update(&instance.0).await.unwrap();

    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&result).unwrap());
}
