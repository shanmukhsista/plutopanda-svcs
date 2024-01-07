use std::any::Any;
use std::fs::metadata;
use axum::extract::{Path, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use axum::{
    response::{IntoResponse},
};
use axum::http::StatusCode;
use chrono::{DateTime, Local, Utc};
use sqlx::Error;
use tracing_subscriber::fmt::format;
use lib_api_common::db::DatabasePool;
use lib_api_common::errors::{ApiError, is_db_row_not_found};

#[derive(Deserialize)]
pub struct LogNewWorkItem {
    pub project_id: i64,
    pub name: String,
    pub date :Option<chrono::DateTime<chrono::Utc>>,
    pub summary: Option<String>,
    pub auto_start_task : Option<bool>,
    pub hours: Option<f64>,
}

#[derive(Deserialize)]
pub struct CompleteWorkItem {
    pub completed_on: Option<chrono::DateTime<chrono::Utc>>,
}
#[derive(Deserialize)]
pub struct StartWorkItem {
    pub started_on: Option<chrono::DateTime<chrono::Utc>>,
}
#[derive(Serialize)]
pub struct WorkItem {
    pub id: i64,
    pub title: String,
    pub summary: Option<String>,
    pub created_on: chrono::DateTime<chrono::Utc>,
    pub started_on: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_on: Option<chrono::DateTime<chrono::Utc>>,
    pub duration_ms: Option<i32>,
    pub time_spent_hours: Option<f64>,
    pub project_id: i64,
}

pub async fn log_work_item(
    State(app_state): State<AppState>,
    Json(request): Json<LogNewWorkItem>,
) -> Result<Json<WorkItem>, ApiError> {
    let mut start_date: Option<chrono::DateTime<chrono::Utc>> = None;
    // check if projct id exists.
    match request.auto_start_task {
        None => {}
        Some(auto_start) => {
            if auto_start {
                start_date = Some(Utc::now())
            }
        }
    }
    let response = sqlx::query_as!(WorkItem,
        "insert into work_items (title, summary, project_id  , started_on,time_spent_hours ) values\
          ($1, $2, $3, $4,$5) returning id , title , summary , created_on , started_on, completed_on, duration_ms,project_id , time_spent_hours",
        request.name.clone(),request.summary.clone(),request.project_id , start_date, request.hours)
        .fetch_one(&app_state.database_pool)
        .await.unwrap();
    tracing::debug!("Created project with id {} successfully. " , response.id);
    Ok(Json(response))
}


pub async fn complete_work_item(
    Path(id): Path<i32>,
    State(app_state): State<AppState>,
    Json(request): Json<CompleteWorkItem>,
) -> Result<Json<WorkItem>, ApiError> {
    let conn =&app_state.database_pool ;
    let existing_project = get_work_item_from_db(id, conn).await?;
    if existing_project.started_on.is_none() {
        return Err(ApiError::bad_request(format!("cannot complete. this project has not started ( id : {} ) " , id).to_string()))
    }
    let  completion_date = match request.completed_on {
        None => {
            Utc::now()
        }
        Some(override_date) => {
            override_date
        }
    };
    let response = sqlx::query_as!(WorkItem,
        "update  work_items set completed_on = $1 where id = $2 returning id , title , summary , created_on , started_on, completed_on, duration_ms,project_id, time_spent_hours",
         completion_date, id)
        .fetch_one(&app_state.database_pool)
        .await.unwrap();
    return Ok(Json(response))
}


pub async fn start_work_item(
    Path(id): Path<i32>,
    State(app_state): State<AppState>,
    Json(request): Json<StartWorkItem>,
) -> Result<Json<WorkItem>, ApiError> {
    let conn =&app_state.database_pool ;
    let existing_project = get_work_item_from_db(id, conn).await?;
    if existing_project.started_on.is_some() {
        return Err(ApiError::bad_request(format!("cannot start. this project has already been started ( id : {} ) " , id).to_string()))
    }
    let  start_date = match request.started_on {
        None => {
            Utc::now()
        }
        Some(override_date) => {
            override_date
        }
    };
    let response = sqlx::query_as!(WorkItem,
        "update  work_items set started_on = $1 where id = $2 returning id , title , summary , created_on , started_on, completed_on, duration_ms,project_id, time_spent_hours",
         start_date,id)
        .fetch_one(&app_state.database_pool)
        .await.unwrap();
    return Ok(Json(response))
}


pub async fn get_work_item_by_id(
    Path(id): Path<i32>,
    State(app_state): State<AppState>,
) -> Result<Json<WorkItem>, ApiError> {
    let existing_project = get_work_item_from_db(id, &app_state.database_pool).await?;
    return Ok(Json(existing_project))
}

async fn get_work_item_from_db(id: i32, pool : &DatabasePool) -> Result<WorkItem, ApiError> {
    let response = sqlx::query_as!(WorkItem,
        "select  id , title , summary , created_on , started_on, completed_on, duration_ms,project_id , time_spent_hours from work_items where id = $1; "
        ,id).fetch_one(pool)
        .await;
    match response {
        Ok(work_item) => {
            tracing::debug!("Created project with id {} successfully. " , work_item.id);
            return Ok(work_item);
        }
        Err(err) => {
            if is_db_row_not_found(err) {
                return Err(ApiError::not_found(format!("work item with id {} not found ", id).to_string()));
            }
            return Err(ApiError::new_internal("unknown error fetching work item".to_string()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::Pool;
    use sqlx::Postgres;
    use crate::handlers::projects::{create_new_project, CreateProjectRequest};

    #[sqlx::test]
    async fn test_create_new_work_item(pool: Pool<Postgres>) {
        let app_state = AppState { database_pool: pool };

        let project = CreateProjectRequest {
            name: "Test Project".to_string()
        };
        let create_project_response = create_new_project(State(app_state.clone()), Json(project)).await;
        assert_eq!(create_project_response.is_ok(), true);

        let created_project = create_project_response.unwrap().0;
        assert_eq!(created_project.id > 0, true);
        assert_eq!(created_project.title.clone(), "Test Project".to_string());

        let request = LogNewWorkItem {
            project_id: created_project.id,
            name: String::from("Test Work Item"),
            summary: Some(String::from("This is a test work item")),
            auto_start_task: Some(true),
            date: None,
            hours: None
        };

        let response = log_work_item(State(app_state.clone()), Json(request)).await;
    }
}