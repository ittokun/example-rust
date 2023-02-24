use actix_web::{web, App, HttpResponse, HttpServer, guard};
use std::sync::Mutex;

mod config;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = move || {
        let counter = web::Data::new(api::AppStateWithCounter {
            app_name: String::from("Actix Web"),
            counter: Mutex::new(0),
        });

        let www_guard = web::scope("/")
            .guard(guard::Header("Host", "www.rust-lang.org"))
            .route("", web::to(|| async { HttpResponse::Ok().body("www") }));
        let user_guard = web::scope("/")
            .guard(guard::Header("Host", "users.rust-lang.org"))
            .route("", web::to(|| async { HttpResponse::Ok().body("user") }));

        let users_scope = web::scope("/users").service(api::show_users);
        let app_scope = web::scope("/app")
            .route("/index.html", web::get().to(api::app));

        App::new()
            .configure(config::config)
            .app_data(counter)
            .service(web::scope("/api").configure(config::api_config))
            .service(www_guard)
            .service(user_guard)
            .service(api::index)
            .service(api::hello)
            .service(api::echo)
            .service(users_scope)
            .service(app_scope)
            .route("/hey", web::get().to(api::manual_hello))
    };

    HttpServer::new(app).bind(("127.0.0.1", 8080))?.run().await
}
