use crate::error::Error;
use actix_web::{get, web, HttpResponse};
use handlebars::Handlebars;

#[get("/")]
pub async fn index_get(hb: web::Data<Handlebars<'_>>) -> Result<HttpResponse, Error> {
    let data = json!({
        "name": "Handlebars"
    });
    let body = hb.render("index", &data)?;

    Ok(HttpResponse::Ok().body(body))
}
