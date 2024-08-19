use actix_web::{web, http, App, HttpServer};
use std::sync::Mutex;
use std::{io, env};


use sqlx::postgres::PgPoolOptions; //用于创建数据库连接池
use dotenv::dotenv;
use actix_cors::Cors;


use routers::*;
use state::AppState;
use crate::errors::MyError;

//服务器
//在这里声明模块
#[path = "../handlers/mod.rs"]
mod handlers;
#[path = "../dbaccess/mod.rs"]
mod dbaccess;

#[path = "../models/mod.rs"]
mod models;

#[path = "../routers.rs"]
mod routers;
#[path = "../errors.rs"]
mod errors;
#[path = "../state.rs"]
mod state;



//实例化 http sever并运行
#[actix_rt::main]
async fn main() -> io::Result<()> 
{
    
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有设置");

    let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();//数据库连接池
    
    //初始化程序状态,在下面被注册到web应用
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),
        //courses: Mutex::new(vec![]),
        db: db_pool,
    });
    println!("shared_data=\n{:?}",shared_data);
    //闭包创建web应用
    let app = move || {
        
        //允许跨域，8080的端口访问
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080/")
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().starts_with(b"http://localhost")
            })
            .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        //构建app，配置route
        App::new()
            .app_data(shared_data.clone()) //注册到web应用
            
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| 
                {
                MyError::InvalidInput("用户非法输入错误".to_string()).into()
                }
            ))
            .wrap(cors)
            
            .configure(general_routes) //配置路由  
            .configure(course_routes)    
            .configure(teacher_routes)
        
    };
    println!("Service is running");

    //运行http server
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
