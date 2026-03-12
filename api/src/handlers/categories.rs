use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;

use crate::AppState;
use shared::{AppError, AppResult, Category, CreateCategoryRequest, UpdateCategoryRequest};

pub async fn list_categories(
    State(state): State<AppState>,
) -> AppResult<Json<Vec<Category>>> {
    let cats = sqlx::query_as!(
        Category,
        "SELECT id, name, description, created_at, updated_at FROM categories ORDER BY name"
    )
    .fetch_all(&state.db)
    .await?;
    Ok(Json(cats))
}

pub async fn get_category(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Category>> {
    let cat = sqlx::query_as!(
        Category,
        "SELECT id, name, description, created_at, updated_at FROM categories WHERE id = $1",
        id
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Category {} not found", id)))?;
    Ok(Json(cat))
}

pub async fn create_category(
    State(state): State<AppState>,
    Json(payload): Json<CreateCategoryRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO categories (id, name, description) VALUES ($1, $2, $3)",
        id, payload.name, payload.description
    )
    .execute(&state.db)
    .await?;
    Ok(Json(serde_json::json!({ "message": "Category created", "id": id })))
}

pub async fn update_category(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCategoryRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let rows = sqlx::query!(
        "UPDATE categories SET name = COALESCE($1, name), description = COALESCE($2, description), updated_at = now() WHERE id = $3",
        payload.name, payload.description, id
    )
    .execute(&state.db)
    .await?
    .rows_affected();

    if rows == 0 { return Err(AppError::NotFound(format!("Category {} not found", id))); }
    Ok(Json(serde_json::json!({ "message": "Category updated" })))
}

pub async fn delete_category(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    sqlx::query!("DELETE FROM categories WHERE id = $1", id)
        .execute(&state.db)
        .await?;
    Ok(Json(serde_json::json!({ "message": "Category deleted" })))
}
