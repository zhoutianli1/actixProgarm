use crate::errors::MyError;

use actix_web::web::{self, Json};
use chrono::NaiveDateTime;  //日期时间类型
use std::convert::TryFrom;
use serde::{Deserialize, Serialize};


/*
    实现sqlx::FromRow ，读取数据库表的时候，通过sqlx::query_as!，可以自动将表数据映射为Course
    只有Serialize序列化，无Deserialize：Course只用来存放数据库读取结果，不涉及新增或者修改，所以只需要从数据库读取后Course序列化为json
*/
#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Course   //课程表
{
   //数据库中无usize、some(T)类型，所以这里teacher_id，id设置为ℹ32
    pub teacher_id: i32,   
    pub id:i32,
    pub name: String,
    pub time: Option<NaiveDateTime>,

    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
    
}

/*
用于新增course：
    实现Deserialize，json转为CreateCourse，插入到数据库
*/
#[derive(Deserialize, Debug, Clone)]
pub struct CreateCourse 
{
    //id自增，time由数据库生成，这里都不需要
    pub teacher_id: i32,
    pub name: String,

    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

//实现TryFrom特征，将 请求中的Json数据转为 CreateCourse，用于新增 ；转换失败返回MyError
impl TryFrom<web::Json<CreateCourse>> for CreateCourse 
{
    type Error = MyError;

    fn try_from(course: Json<CreateCourse>) -> Result<Self, Self::Error> 
    {
        Ok(CreateCourse {
            teacher_id: course.teacher_id,
            name: course.name.clone(),
            description: course.description.clone(),
            format: course.format.clone(),
            structure: course.structure.clone(),
            duration: course.duration.clone(),
            price: course.price,
            language: course.language.clone(),
            level: course.level.clone(),
        })
    }
}


#[derive(Deserialize, Debug, Clone)]
pub struct UpdateCourse {
    pub name: Option<String>,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

impl TryFrom<web::Json<UpdateCourse>> for UpdateCourse {
    type Error = MyError;

    fn try_from(course: Json<UpdateCourse>
    ) -> Result<Self, Self::Error> {
        Ok(UpdateCourse {
            name: course.name.clone(),
            description: course.description.clone(),
            format: course.format.clone(),
            structure: course.structure.clone(),
            duration: course.duration.clone(),
            price: course.price,
            language: course.language.clone(),
            level: course.level.clone(),
        })
    }
}