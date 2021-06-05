use super::super::app_state::AppState;
use actix_web::http::{ContentEncoding, StatusCode};
use actix_web::{http, web, HttpRequest, HttpResponse};

#[allow(dead_code)]
pub async fn resource_query(data: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let query = req.match_info().get("query").unwrap_or("");
    let page = req.match_info().get("page").unwrap_or("0");
    let page = page.parse::<u16>().unwrap();

    let resources = &data.bd.lock().unwrap().select_resources(query, 10, page);

    if resources.len() == 0 {
        return HttpResponse::build(StatusCode::NOT_FOUND)
            .set_header(http::header::CONTENT_TYPE, "text/json")
            .set_header(
                http::header::CONTENT_ENCODING,
                ContentEncoding::Identity.as_str(),
            )
            .finish();
    }
    return HttpResponse::build(StatusCode::OK)
        .set_header(http::header::CONTENT_TYPE, "text/json")
        .set_header(
            http::header::CONTENT_ENCODING,
            ContentEncoding::Identity.as_str(),
        )
        .json(&resources);
}
