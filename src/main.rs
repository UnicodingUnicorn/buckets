use actix_web::{ get, App, HttpResponse, HttpServer, Responder };
use actix_web::web::Data;
use handlebars::Handlebars;
use std::collections::BTreeMap;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("main", "./templates/main.hbs").unwrap();

    let handlebars = Arc::new(handlebars);
    HttpServer::new(move || {
        App::new().data(handlebars.clone())
            .service(index)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[get("/")]
async fn index(handlebars:Data<Arc<Handlebars<'_>>>) -> impl Responder {
    let data:BTreeMap<String, String> = BTreeMap::new();

    HttpResponse::Ok().body(handlebars.render("main", &data).unwrap())
}
