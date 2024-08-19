#[path = "../mod.rs"]
mod wa;

use wa::{errors, handlers, models, routers};

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use routers::app_config;
use std::env;

use tera::Tera;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let host_url = env::var("HOST_PORT").expect("HOST_URL not set"); //地址端口，HOST_PORT=127.0.0.1:8080
    println!("Listening on: {}", &host_url);

    HttpServer::new(move || 
        {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*")).unwrap();

        //构建app，配置route
        App::new().app_data(web::Data::new(tera))
        .configure(app_config) //配置路由
        })
        .bind(&host_url)?// //运行http server
        .run()
        .await
}