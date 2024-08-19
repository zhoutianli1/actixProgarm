use actix_web::{error, http::StatusCode, HttpResponse, Result};
use sqlx::error::Error as SQLxError;
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
//自定义错误类型
pub enum MyError {
    DBError(String),
    ActixError(String),
    NotFound(String),
    InvalidInput(String), //非法输入错误
}
//返回给用户的错误响应
#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

//错误发生，需要将 MyError转为 MyErrorResponse

//1.MyError转为 字符串消息
impl MyError {
    fn error_response(&self) -> String {
        match self {
            MyError::DBError(msg) => {
                println!("数据库错误-Database error occurred: {:?}", msg);
                "Database error".into()
            },
            MyError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            },
            MyError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            },
            //非法输入错误
            MyError::InvalidInput(msg) => {
                println!("Invalid paramters received: {:?}", msg);
                msg.into()
            }
        }
    }
}

//实现error::ResponseError 特征，需要对应结构体实现debug 和Display
//2.状态码 和 字符串消息--》HttpResponse
impl error::ResponseError for MyError {
    fn status_code(&self) -> StatusCode  //状态码
    {
        match self {
            MyError::DBError(_msg) | MyError::ActixError(_msg) => StatusCode::INTERNAL_SERVER_ERROR, //500
            MyError::NotFound(_msg) => StatusCode::NOT_FOUND,       //404
            MyError::InvalidInput(_msg) => StatusCode::BAD_REQUEST,  //400
        }
    }
    //
    fn error_response(&self) -> HttpResponse 
    {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {error_message: self.error_response(),})
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}
 
//让actix_web::error 和 SQLxError 可以转为 MyError
impl From<actix_web::error::Error> for MyError {
    fn from(err: actix_web::error::Error) -> Self {
        MyError::ActixError(err.to_string())
    }
}

impl From<SQLxError> for MyError {
    fn from(err: SQLxError) -> Self {
        MyError::DBError(err.to_string())
    }
}