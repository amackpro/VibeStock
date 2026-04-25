use axum::{
    extract::{Extension, Path, State},
    Json,
};
use uuid::Uuid;

use crate::AppState;
use shared::{AppError, AppResult, Category, CreateCategoryRequest, UpdateCategoryRequest};

/// GET /api/categories — list all categories ordered by name
pub async fn list_categories(
    State(state): State<AppState>,
    Extension(tenant_id): Extension<Uuid>,
) -> AppResult<Json<Vec<Category>>> {
    let cats = sqlx::query_as::<_, Category>(
        "SELECT id, tenant_id, name, description, created_at, updated_at \
         FROM categories WHERE tenant_id = $1 ORDER BY name"
    )
    .bind(tenant_id)
    .fetch_all(&state.db)
    .await?;
    Ok(Json(cats))
}

/// GET /api/categories/:id
pub async fn get_category(
    State(state): State<AppState>,
    Extension(tenant_id): Extension<Uuid>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Category>> {
    let cat = sqlx::query_as::<_, Category>(
        "SELECT id, tenant_id, name, description, created_at, updated_at \
         FROM categories WHERE id = $1 AND tenant_id = $2"
    )
    .bind(id)
    .bind(tenant_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Category {} not found", id)))?;
    Ok(Json(cat))
}

/// POST /api/categories
pub async fn create_category(
    State(state): State<AppState>,
    Extension(tenant_id): Extension<Uuid>,
    Json(payload): Json<CreateCategoryRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO categories (id, tenant_id, name, description) VALUES ($1, $2, $3, $4)"
    )
    .bind(id)
    .bind(tenant_id)
    .bind(&payload.name)
    .bind(&payload.description)
    .execute(&state.db)
    .await?;
    Ok(Json(serde_json::json!({ "message": "Category created", "id": id })))
}

/// PUT /api/categories/:id
pub async fn update_category(
    State(state): State<AppState>,
    Extension(tenant_id): Extension<Uuid>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCategoryRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let rows = sqlx::query(
        "UPDATE categories SET \
         name        = COALESCE($1::text, name), \
         description = COALESCE($2::text, description), \
         updated_at  = now() \
         WHERE id = $3 AND tenant_id = $4"
    )
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(id)
    .bind(tenant_id)
    .execute(&state.db)
    .await?
    .rows_affected();

    if rows == 0 {
        return Err(AppError::NotFound(format!("Category {} not found", id)));
    }
    Ok(Json(serde_json::json!({ "message": "Category updated" })))
}

/// DELETE /api/categories/:id
pub async fn delete_category(
    State(state): State<AppState>,
    Extension(tenant_id): Extension<Uuid>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    let rows = sqlx::query("DELETE FROM categories WHERE id = $1 AND tenant_id = $2")
        .bind(id)
        .bind(tenant_id)
        .execute(&state.db)
        .await?
        .rows_affected();

    if rows == 0 {
        return Err(AppError::NotFound(format!("Category {} not found", id)));
    }
    Ok(Json(serde_json::json!({ "message": "Category deleted" })))
}
