use axum::{
    extract::{Extension, Path, State},
    Json,
};
use uuid::Uuid;

use crate::AppState;
use shared::{
    AppError, AppResult, City, CityDashboardStats, CityWithCountry, Country, CountryStats, Region,
    RegionStats, SupplierSummary,
};

// ─── Public Endpoints (No Auth) ───────────────────────────────────────────────

/// GET /api/geography/regions — List all regions
pub async fn list_regions(State(state): State<AppState>) -> AppResult<Json<Vec<Region>>> {
    let regions = sqlx::query_as::<_, Region>("SELECT * FROM regions ORDER BY name ASC")
        .fetch_all(&state.db)
        .await?;

    Ok(Json(regions))
}

/// GET /api/geography/regions/:id/countries — List countries in a region
pub async fn list_countries_by_region(
    State(state): State<AppState>,
    Path(region_id): Path<Uuid>,
) -> AppResult<Json<Vec<Country>>> {
    let countries = sqlx::query_as::<_, Country>(
        "SELECT * FROM countries WHERE region_id = $1 ORDER BY name ASC",
    )
    .bind(region_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(countries))
}

/// GET /api/geography/countries/:id/cities — List cities in a country
pub async fn list_cities_by_country(
    State(state): State<AppState>,
    Path(country_id): Path<Uuid>,
) -> AppResult<Json<Vec<City>>> {
    let cities = sqlx::query_as::<_, City>(
        "SELECT * FROM cities WHERE country_id = $1 ORDER BY population DESC, name ASC LIMIT 100",
    )
    .bind(country_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(cities))
}

/// GET /api/geography/cities/:id — Get city details
pub async fn get_city(
    State(state): State<AppState>,
    Path(city_id): Path<Uuid>,
) -> AppResult<Json<CityWithCountry>> {
    let city = sqlx::query_as::<_, CityWithCountry>(
        "SELECT ci.id, ci.country_id, co.name AS country_name, co.iso2 AS country_iso2,
                ci.name, ci.state_name, ci.latitude, ci.longitude, ci.population,
                ci.is_capital, ci.created_at
         FROM cities ci
         JOIN countries co ON co.id = ci.country_id
         WHERE ci.id = $1",
    )
    .bind(city_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::NotFound("City not found".into()))?;

    Ok(Json(city))
}

// ─── Protected Endpoints (Requires Tenant) ────────────────────────────────────

/// GET /api/geography/regions/:id/stats — Region stats for current tenant
pub async fn get_region_stats(
    State(state): State<AppState>,
    Extension(tenant_id): Extension<Uuid>,
    Path(region_id): Path<Uuid>,
) -> AppResult<Json<RegionStats>> {
    let stats = sqlx::query_as::<_, RegionStats>(
        "SELECT
            r.id AS region_id,
            r.name AS region_name,
            COUNT(DISTINCT co.id) AS country_count,
            COUNT(DISTINCT ci.id) AS city_count,
            COALESCE(COUNT(DISTINCT s.id), 0) AS supplier_count
         FROM regions r
         LEFT JOIN countries co ON co.region_id = r.id
         LEFT JOIN cities ci ON ci.country_id = co.id
         LEFT JOIN suppliers s ON s.city_id = ci.id AND s.tenant_id = $1
         WHERE r.id = $2
         GROUP BY r.id, r.name",
    )
    .bind(tenant_id)
    .bind(region_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::NotFound("Region not found".into()))?;

    Ok(Json(stats))
}

/// GET /api/geography/countries/:id/stats — Country stats for current tenant
pub async fn get_country_stats(
    State(state): State<AppState>,
    Extension(tenant_id): Extension<Uuid>,
    Path(country_id): Path<Uuid>,
) -> AppResult<Json<CountryStats>> {
    let stats = sqlx::query_as::<_, CountryStats>(
        "SELECT
            co.id AS country_id,
            co.name AS country_name,
            COUNT(DISTINCT ci.id) AS city_count,
            COALESCE(COUNT(DISTINCT s.id), 0) AS supplier_count
         FROM countries co
         LEFT JOIN cities ci ON ci.country_id = co.id
         LEFT JOIN suppliers s ON s.city_id = ci.id AND s.tenant_id = $1
         WHERE co.id = $2
         GROUP BY co.id, co.name",
    )
    .bind(tenant_id)
    .bind(country_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::NotFound("Country not found".into()))?;

    Ok(Json(stats))
}

/// GET /api/geography/cities/:id/dashboard — Full city dashboard stats for current tenant
pub async fn get_city_dashboard_stats(
    State(state): State<AppState>,
    Extension(tenant_id): Extension<Uuid>,
    Path(city_id): Path<Uuid>,
) -> AppResult<Json<CityDashboardStats>> {
    // First verify city exists
    let city_name: Option<String> = sqlx::query_scalar("SELECT name FROM cities WHERE id = $1")
        .bind(city_id)
        .fetch_optional(&state.db)
        .await?;

    let city_name = city_name.ok_or(AppError::NotFound("City not found".into()))?;

    // Count suppliers in city for tenant
    let total_suppliers: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM suppliers WHERE city_id = $1 AND tenant_id = $2",
    )
    .bind(city_id)
    .bind(tenant_id)
    .fetch_one(&state.db)
    .await?;

    // Count products via suppliers in city
    let total_products: i64 = sqlx::query_scalar(
        "SELECT COUNT(DISTINCT p.id) FROM products p
         JOIN suppliers s ON p.supplier_id = s.id
         WHERE s.city_id = $1 AND s.tenant_id = $2",
    )
    .bind(city_id)
    .bind(tenant_id)
    .fetch_one(&state.db)
    .await?;

    // Total stock value
    let total_stock_value: f64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(p.unit_price * p.quantity_in_stock::float8), 0.0)
         FROM products p
         JOIN suppliers s ON p.supplier_id = s.id
         WHERE s.city_id = $1 AND s.tenant_id = $2 AND p.is_active = true",
    )
    .bind(city_id)
    .bind(tenant_id)
    .fetch_one(&state.db)
    .await?;

    // Low stock count
    let low_stock_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM products p
         JOIN suppliers s ON p.supplier_id = s.id
         WHERE s.city_id = $1 AND s.tenant_id = $2
           AND p.is_active = true
           AND p.quantity_in_stock > 0
           AND p.quantity_in_stock <= p.reorder_level",
    )
    .bind(city_id)
    .bind(tenant_id)
    .fetch_one(&state.db)
    .await?;

    // Out of stock count
    let out_of_stock_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM products p
         JOIN suppliers s ON p.supplier_id = s.id
         WHERE s.city_id = $1 AND s.tenant_id = $2
           AND p.is_active = true
           AND p.quantity_in_stock = 0",
    )
    .bind(city_id)
    .bind(tenant_id)
    .fetch_one(&state.db)
    .await?;

    // Recent movements (last 7 days)
    let recent_movements: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM stock_movements sm
         JOIN products p ON p.id = sm.product_id
         JOIN suppliers s ON p.supplier_id = s.id
         WHERE s.city_id = $1 AND s.tenant_id = $2
           AND sm.created_at >= NOW() - INTERVAL '7 days'",
    )
    .bind(city_id)
    .bind(tenant_id)
    .fetch_one(&state.db)
    .await?;

    // Get list of suppliers in this city with their product counts
    let suppliers = sqlx::query_as::<_, SupplierSummary>(
        "SELECT
            s.id,
            s.name,
            s.contact_name,
            s.email,
            s.phone,
            COUNT(DISTINCT p.id) as product_count
         FROM suppliers s
         LEFT JOIN products p ON p.supplier_id = s.id AND p.is_active = true
         WHERE s.city_id = $1 AND s.tenant_id = $2
         GROUP BY s.id, s.name, s.contact_name, s.email, s.phone
         ORDER BY s.name",
    )
    .bind(city_id)
    .bind(tenant_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(CityDashboardStats {
        city_id,
        city_name,
        total_suppliers,
        total_products,
        total_stock_value,
        low_stock_count,
        out_of_stock_count,
        recent_movements,
        suppliers,
    }))
}

/// GET /api/geography/cities-with-inventory — Only cities that have suppliers/products for current tenant
pub async fn get_cities_with_inventory(
    State(state): State<AppState>,
    Extension(tenant_id): Extension<Uuid>,
) -> AppResult<Json<Vec<CityWithCountry>>> {
    let cities = sqlx::query_as::<_, CityWithCountry>(
        "SELECT DISTINCT
            ci.id, ci.country_id, co.name AS country_name, co.iso2 AS country_iso2,
            ci.name, ci.state_name, ci.latitude, ci.longitude, ci.population,
            ci.is_capital, ci.created_at
         FROM cities ci
         JOIN countries co ON co.id = ci.country_id
         JOIN suppliers s ON s.city_id = ci.id
         WHERE s.tenant_id = $1
         ORDER BY ci.population DESC, ci.name ASC",
    )
    .bind(tenant_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(cities))
}
