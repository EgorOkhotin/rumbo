use actix_web::http::header::{HeaderMap, HeaderValue};

use super::prelude::*;

#[derive(Deserialize)]
pub struct LoginDto {
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct RegistrateDto {
    email: String,
    name: String,
    password: String,
}

#[post("api/session")]
pub async fn login(app: web::Data<RumboApp>, info: web::Json<LoginDto>) -> impl Responder {
    info!("User trying to login");

    let service = &app.users_service;

    let result = service.authenticate(&info.email, &info.password).await;

    if let Ok(user) = result {
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&user).unwrap());
    };

    HttpResponse::NotFound()
        .content_type(ContentType::plaintext())
        .body("Authorization failed")
}

#[delete("api/session")]
pub async fn logout(_app: web::Data<RumboApp>, request: HttpRequest) -> impl Responder {
    let user = get_auth_info(request);

    if user.is_none() {
        return HttpResponse::Unauthorized()
            .content_type(ContentType::json())
            .body("Auth data was incorrect!");
    }

    let user = user.unwrap();
    info!("Trying to logout user {}", &user.email);

    todo!("Revoke token");

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(())
}

#[post("api/user")]
pub async fn registrate_user(
    app: web::Data<RumboApp>,
    info: web::Json<RegistrateDto>,
) -> impl Responder {
    info!("Trying to registrate new user");

    let service = &app.users_service;
    let result = service
        .add_new_user(
            info.name.to_string(),
            info.email.to_string(),
            info.password.to_string(),
        )
        .await;

    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&result).unwrap());
}

#[delete("api/user")]
pub async fn delete_user(app: web::Data<RumboApp>, request: HttpRequest) -> impl Responder {
    let user = get_auth_info(request);

    if user.is_none() {
        return HttpResponse::Unauthorized()
            .content_type(ContentType::json())
            .body("Auth data was incorrect!");
    }
    let user = user.unwrap();

    info!("Trying to delete user with email={}", &user.email);

    let service = &app.users_service;
    let _result = service.delete_user(&user.email);

    todo!("Revoke token");

    // return HttpResponse::Ok().content_type(ContentType::json()).body();
}

#[patch("api/user")]
pub async fn update_user(app: web::Data<RumboApp>, user: web::Json<User>) -> impl Responder {
    info!("Trying to update user");

    let service = &app.users_service;
    let user = service.update_user(user.into_inner()).await;

    match user {
        Err(reason) => HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&reason).unwrap()),
        Ok(user) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&user).unwrap()),
    }
}

fn get_auth_header(headers: &HeaderMap) -> Option<&HeaderValue> {
    headers.get("Authorization")
}

fn get_auth_info(request: HttpRequest) -> Option<User> {
    let header = get_auth_header(request.headers())?;

    let string_token = header.to_str();
    if string_token.is_err() {
        return None;
    }

    let _token = string_token.unwrap().to_string();

    todo!("Add auth")
}
