use crate::errors::MyError;
use crate::models::teacher::{CreateTeacher, UpdateTeacher, Teacher};

use sqlx::postgres::PgPool;

//1.获得所有老师
pub async fn get_all_teacher_db(
    pool: &PgPool
) -> Result<Vec<Teacher>, MyError> 
{
    //大多数场景还是使用query而非query_as,因为不可能和Struct字段一一对应的
    //.fetch_all(...)：查询所有
    let rows = sqlx::query!("SELECT * FROM teacher")
        .fetch_all(pool)
        .await?;

    //rows：日志数组
    //rows=[Record { id: 1, name: "王老师", picture_url: "/d/picture1", profile: Some("撒打算大") }]
    //为什么 profile: Some("撒打算大") 
    println!("从数据库查询所有老师的信息，日志rows={:?}",&rows);

    //日志数组 转为 Vec<Teacher>
    let teachers: Vec<Teacher> = rows
        .iter()
        .map(|r| Teacher {
            id: r.id,
            name: r.name.clone(),
            picture_url: r.picture_url.clone(),
            profile: r.profile.clone().unwrap(),
            //profile: r.profile.clone(),
        })
        .collect();

    match teachers.len() {
        0 => Err(MyError::NotFound("Not teachers found".into())),
        _ => Ok(teachers),
    }
}
//2.获取某个老师信息
pub async fn get_teacher_details_db(
    pool: &PgPool,
    teacher_id: i32,
) -> Result<Teacher, MyError> 
{
    //.fetch_one(数据库连接池).await：查询一个
    let rows = sqlx::query!(
        "SELECT * FROM teacher where id = $1", teacher_id)
        .fetch_one(pool)
        .await
        .map(|r| Teacher {
            id: r.id,
            name: r.name.clone(),
            picture_url: r.picture_url.clone(),
            //profile: r.profile.clone(),
            profile: r.profile.clone().unwrap(),
        })
        .map_err(|_err| MyError::NotFound("Teacher Id not found".into()))?;

    Ok(rows)
}

//3.
pub async fn post_new_teacher_db(
    pool: &PgPool,
    new_teacher: CreateTeacher,
) -> Result<Teacher, MyError> {
    let row = sqlx::query!("INSERT INTO teacher (name, picture_url, profile) VALUES ($1, $2, $3) RETURNING id, name, picture_url, profile",
        new_teacher.name,
        new_teacher.picture_url,
        new_teacher.profile)
        .fetch_one(pool)
        .await?;

    Ok(Teacher {
        id: row.id,
        name: row.name,
        picture_url: row.picture_url,
        profile: row.profile.unwrap()
        //profile: row.profile
    })
}

//4.
pub async fn update_teacher_details_db(
    pool: &PgPool,
    teacher_id: i32,
    update_teacher: UpdateTeacher,
) -> Result<Teacher, MyError> {
    let row = sqlx::query!("SELECT * FROM teacher where id = $1", teacher_id)
        .fetch_one(pool)
        .await
        .map_err(|_err| MyError::NotFound("Teacher Id not found".into()))?;

    let temp = Teacher {
        id: row.id,
        name: if let Some(name) = update_teacher.name {
            name
        } else {
            row.name
        },
        picture_url: if let Some(picture_url) = update_teacher.picture_url {
            picture_url
        } else {
            row.picture_url
        },
        profile: if let Some(profile) = update_teacher.profile {
            profile
        } else {
            row.profile.unwrap()
            //row.profile
        },
    };

    let update_row = sqlx::query!(
        "UPDATE teacher SET name = $1, picture_url = $2, profile = $3 WHERE  id = $4 RETURNING id, name, picture_url, profile",
        temp.name,
        temp.picture_url,
        temp.profile,
        teacher_id
    )
        .fetch_one(pool)
        .await
        .map(|r| Teacher {
            id: r.id,
            name: r.name,
            picture_url: r.picture_url,
            profile: r.profile.unwrap(),
        })
        .map_err(|_err| MyError::NotFound("Teacher id not found".into()))?;
    Ok(update_row)
}

pub async fn delete_teacher_db(
    pool: &PgPool,
    teacher_id: i32,
) -> Result<String, MyError> {
    let row = sqlx::query!(
        "DELETE FROM teacher WHERE id = $1",
        teacher_id
    )   .execute(pool)
        .await
        .map_err(|_err| MyError::DBError("Unable to delete teacher".into()))?;

    Ok(format!("Deleted {:?} record", row))
}