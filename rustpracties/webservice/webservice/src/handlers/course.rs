use crate::errors::MyError;
use crate::models::course::UpdateCourse;
use crate::state::AppState;
use crate::{db_access::course::*, models::course::CreateCourse};
use actix_web::{web, HttpResponse};

pub async fn post_new_course(
    new_course: web::Json<CreateCourse>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    post_new_course_db(&app_state.db, new_course.try_into()?)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn get_course_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let teacher_id = params.into_inner();
    get_courses_for_teacher_db(&app_state.db, teacher_id)
        .await
        .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    get_course_detail_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn delete_course(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    delete_course_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn update_course_details(
    app_state: web::Data<AppState>,
    update_course: web::Json<UpdateCourse>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    update_course_details_db(
        &app_state.db,
        teacher_id,
        course_id,
        update_course.try_into()?,
    )
    .await
    .map(|course| HttpResponse::Ok().json(course))
}
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::ResponseError;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;
    use std::sync::Mutex;

    #[ignore]
    #[actix_rt::test]
    async fn post_course_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("database url is not set");
        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();
        let course = web::Json(CreateCourse {
            teacher_id: 1,
            name: "Test course".into(),
            description: Some("tests".into()),
            format: Some("tests".into()),
            structure: Some("tests".into()),
            duration: Some("tests".into()),
            price: Some(123),
            language: Some("tests".into()),
            level: Some("tests".into()),
        });
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let resp = post_new_course(course, app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_courses_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("database url is not set");
        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let teacher_id: web::Path<i32> = web::Path::from(1);
        let resp = get_course_for_teacher(app_state, teacher_id).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_courses_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("database url is not set");
        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let params: web::Path<(i32, i32)> = web::Path::from((1, 1));
        let resp = get_course_detail(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[actix_rt::test]
    async fn get_one_courses_failure() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("database url is not set");
        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let params: web::Path<(i32, i32)> = web::Path::from((1, 100));
        let resp = get_course_detail(app_state, params).await;
        match resp {
            Ok(_) => println!("something went wrong"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }
    #[actix_rt::test]
    async fn update_courses_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("database url is not set");
        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let update_course = UpdateCourse {
            name: Some("course name changed".into()),
            description: Some("(change)".into()),
            format: Some("(change)".into()),
            structure: Some("(change)".into()),
            duration: Some("(change)".into()),
            price: Some(127),
            language: Some("(change)".into()),
            level: Some("(change)".into()),
        };
        let params: web::Path<(i32, i32)> = web::Path::from((1, 1));
        let update_params = web::Json(update_course);
        update_course_details(app_state, update_params, params)
            .await
            .unwrap();
    }
    #[ignore]
    #[actix_rt::test]
    async fn delete_courses_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("database url is not set");
        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let params: web::Path<(i32, i32)> = web::Path::from((1, 3));
        let resp = delete_course(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[actix_rt::test]
    async fn delete_courses_failure() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("database url is not set");
        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let params: web::Path<(i32, i32)> = web::Path::from((1, 100));
        let resp = delete_course(app_state, params).await;
        match resp {
            Ok(_) => println!("something went wrong"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }
}
