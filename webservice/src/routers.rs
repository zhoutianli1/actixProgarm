use crate::handlers::{course, general, teacher};
use actix_web::web;


//健康检查 路由 
pub fn general_routes(cfg: &mut web::ServiceConfig) {

    cfg.route("/health", web::get().to(general::health_check_handler)); //处理逻辑：handler::health_check_handler函数
}

//course 的相关路径
pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::scope("/courses") //service定义一个作用域，下面所有路径都在“/courses”下进行
            .route("/", web::post().to(course::post_new_course))
            .route("/{teacher_id}", web::get().to(course::get_courses_for_teacher))
            .route("/{teacher_id}/{course_id}", web::get().to(course::get_course_detail))
        
            .route("/{teacher_id}", web::get().to(course::get_courses_for_teacher))
            
            .route("/{teacher_id}/{course_id}", web::delete().to(course::delete_course))
            .route("/{teacher_id}/{course_id}", web::put().to(course::update_course_details))
            
        );
}

//teacher 相关路由
pub fn teacher_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::scope("/teacher")
            .route("/", web::get().to(teacher::get_all_teachers))
            .route("/", web::post().to(teacher::post_new_teacher))
            .route("/{teacher_id}", web::get().to(teacher::get_teacher_details))
            .route("/{teacher_id}", web::put().to(teacher::update_teacher_details))
            .route("/{teacher_id}", web::delete().to(teacher::delete_teacher))
        );
}