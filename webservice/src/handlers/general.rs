use crate::state::AppState;

use actix_web::{web, HttpResponse};

//健康检查处理 handler
pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse 
{
    println!("app_state={:?}",app_state);

    let heath_check_response = &app_state.health_check_response;
    //访问次数字段，当前线程访问前，必须lock防止其他线程访问
    let mut visit_count = app_state.visit_count.lock().unwrap();

    let response =
        format!("{} {} times", heath_check_response, visit_count);
    *visit_count += 1;

    //返回响应
    HttpResponse::Ok().json(&response)
}