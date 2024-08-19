use crate::handlers::{get_all_teachers, handle_register, show_register_form};

use actix_files as fs;
use actix_web::web;

pub fn app_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("")
                //静态文件：客户端访问/static，就去找 ./static下的文件
                .service(fs::Files::new("/static", "./static").show_files_listing())
                //  /对应localhost:8080
                .service(web::resource("/").route(web::get().to(get_all_teachers)))
                // teacher、注册页面
                .service(web::resource("/register").route(web::get().to(show_register_form)))
                //提交表单对应路由
                .service(web::resource("/register-post").route(web::post().to(handle_register))),
        );

}