use super::super::app_state::AppState;
use actix_web::{error, web, HttpRequest, HttpResponse};

#[allow(dead_code)]
pub async fn index(
    tmpl: web::Data<tera::Tera>,
    _: web::Data<AppState>,
    _: HttpRequest,
) -> HttpResponse {
    let s = tmpl
        .render("under_construction.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Template error"))
        .unwrap();

    return HttpResponse::Ok().content_type("text/html").body(s);
}
