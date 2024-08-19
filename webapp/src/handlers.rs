use crate::errors::MyError;
use crate::models::{TeacherRegisterFrom, TeacherResponse};

use actix_web::{web, Error, HttpResponse, Result};
use serde_json::json;


pub async fn get_all_teachers(
    tmpl: web::Data<tera::Tera>
) -> Result<HttpResponse, Error> 
{   
    //创建http客户端
    let awc_client = awc::Client::default();

    //用于访问 teacher Service服务器，获取 返回结果
    let res = awc_client
        .get("http://localhost:3000/teacher/")//获取所有teacher是get请求
        .send()
        .await
        .unwrap()
        .json::<Vec<TeacherResponse>>()//unwrap()返回teacher集合的json，然后转化为Vec<TeacherResponse>
        .await
        .unwrap();

    let mut ctx = tera::Context::new(); //使用上下文向html模版里面添加数据
    ctx.insert("error", "");
    ctx.insert("teachers", &res);  //添加 返回结果

    //使用tera::Tera渲染模版，将数据ctx渲染到teachers.html, 通过{{}}
    let s = tmpl
        .render("teachers.html", &ctx)
        .map_err(|_| MyError::TeraError("Template Error".to_string()))?; //若渲染发生错误，转化为自定义的错误类型
    
    Ok(HttpResponse::Ok().content_type("text/html").body(s)) //将渲染页面返回
}

//返回一个注册页面
pub async fn show_register_form(
    tmpl: web::Data<tera::Tera>
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("corrent_name", "");
    ctx.insert("current_imageurl", "");
    ctx.insert("current_profile", "");
    let s = tmpl
        .render("register.html", &ctx)
        .map_err(|_| MyError::TeraError("Template Error 1".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

//老师注册提交表单，需要的操作
pub async fn handle_register(
    tmpl: web::Data<tera::Tera>,
    params: web::Form<TeacherRegisterFrom>,  //老师注册提交的表单数据
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let s;
    if params.name == "Dave" {
        ctx.insert("error", "Dave already exists!");

        ctx.insert("corrent_name", &params.name);
        ctx.insert("current_imageurl", &params.imageurl);
        ctx.insert("current_profile", &params.profile);

        s = tmpl
            .render("register.html", &ctx)
            .map_err(|_| MyError::TeraError("Template Error".to_string()))?;
    } else {
        let new_teacher = json!({
            "name": &params.name,
            "picture_url": &params.imageurl,
            "profile": &params.profile
        });
        let awc_client = awc::Client::default();

        let res = awc_client
            .post("http://localhost:3000/teacher/")  //发送到techer service服务器的 该路径
            .send_json(&new_teacher)
            .await
            .unwrap()
            .body()
            .await?;

        let teacher_response: TeacherResponse = serde_json::from_str(&std::str::from_utf8(&res)?)?;
        s = format!("Congratulations! Your Id is: {}.", teacher_response.id);
    }
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}