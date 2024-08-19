use serde::{Deserialize, Serialize};


//用于老师注册提交的表单数据
#[derive(Serialize, Deserialize, Debug)]
pub struct TeacherRegisterFrom {
    pub name: String,
    pub imageurl: String,
    pub profile: String,
}

//查询老师返回的结果
#[derive(Serialize, Deserialize, Debug)]
pub struct TeacherResponse {
    pub id: i32,
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}