use axum::extract::State;
use axum::Json;
use axum::response::Response;
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use axum::{
    response::{IntoResponse},
};
use axum::http::StatusCode;
use lib_api_common::errors::ApiError;

#[derive(Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct Project {
    pub id: i64,
    pub title: String,
    pub is_complete : Option<bool>,
    pub created_on : chrono::DateTime<chrono::Utc>,
    pub updated_on : Option<chrono::DateTime<chrono::Utc>>
}

pub async fn create_new_project(
    State(app_state): State<AppState>,
    Json(request): Json<CreateProjectRequest>,
) -> Result<Json<Project>, ApiError>{
    let response = sqlx::query_as!(Project,
        "insert into projects (title) values\
          ($1) returning id , title , is_complete, created_on, updated_on",
        request.name.clone())

        .fetch_one(&app_state.database_pool)
        .await.unwrap();
    tracing::debug!("Created project with id {} successfully. " , response.id) ;
    Ok(Json(response))
}

pub async fn get_all_projects(
    State(app_state): State<AppState>
) -> Response{
    let projects = sqlx::query_as!(Project,
        " select id , title , is_complete, created_on, updated_on from projects")
        .fetch_all(&app_state.database_pool)
        .await.unwrap();
    tracing::debug!("Fetched {} projects. " , projects.len()) ;
    Json(projects).into_response()
}