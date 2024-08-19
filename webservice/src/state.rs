use std::sync::Mutex;
use openssl::derive;
//use super::models::Course;
use sqlx::postgres::PgPool;


//声明：应用程序状态
#[derive( Debug)]
pub struct AppState {
    //健康检查的响应【共享与所有线程】，不可变
    pub health_check_response: String, 

    //访问次数，共【享与所有线程】可变，Mutex保证线程之间通信
    pub visit_count: Mutex<u32>,
    //pub courses: Mutex<Vec<Course>>, //使用内存存储
    pub db: PgPool,//数据库连接池（用于在多线程中共享）             //使用数据库存储
}