use std::sync::Arc;

use axum::{
    extract::DefaultBodyLimit,
    middleware::{self, from_fn},
    routing::{delete, get, patch, post},
    Router,
};
use tokio::sync::broadcast;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
    set_header::SetResponseHeaderLayer,
};
use axum::http::header::{
    X_CONTENT_TYPE_OPTIONS,
    X_FRAME_OPTIONS,
    STRICT_TRANSPORT_SECURITY,
    HeaderValue,
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use api::{
    auth::{auth_middleware, require_admin},
    config::Config,
    db,
    handlers::{
        auth_handler::{health, login, register},
        categories::{create_category, delete_category, get_category, list_categories, update_category},
        dashboard::get_stats,
        geography::{
            get_city, get_city_dashboard_stats, get_cities_with_inventory, get_country_stats,
            get_region_stats, list_cities_by_country, list_countries_by_region, list_regions,
        },
        products::{
            create_product, delete_product, get_product, get_product_by_barcode,
            list_products, update_product,
        },
        stock_movements::{create_movement, list_movements},
        suppliers::{create_supplier, delete_supplier, get_supplier, list_suppliers, update_supplier},
        tenants::{create_tenant, delete_tenant, get_tenant, list_tenants, update_tenant},
        websocket::ws_handler,
        users::{list_users, update_user_role, toggle_user_status},
        reports::{get_inventory_report, get_low_stock_report, get_movements_report, get_valuation_report},
    },
    AppState, InnerState,
};
use shared::WsEvent;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "api=debug,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env()?;
    
    let db = match db::connect(&config.database_url).await {
        Ok(pool) => {
            let reset_db = std::env::var("RESET_DB").unwrap_or_else(|_| "false".to_string());
            if reset_db == "true" || reset_db == "1" {
                tracing::info!("Dropping public schema to reset database because RESET_DB is set...");
                let _ = sqlx::query("DROP SCHEMA public CASCADE; CREATE SCHEMA public;").execute(&pool).await;
            }
            
            if let Err(e) = sqlx::migrate!("./migrations").run(&pool).await {
                tracing::warn!("Failed to apply migrations: {}", e);
            } else {
                tracing::info!("✅ Migrations applied");
            }
            pool
        },
        Err(e) => {
            tracing::error!("Failed to connect to DB: {}", e);
            return Err(e.into());
        }
    };

    let (ws_tx, _) = broadcast::channel::<WsEvent>(128);

    let state: AppState = Arc::new(InnerState { db, config: config.clone(), ws_tx });

    let frontend_url = std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());
    
    let cors = CorsLayer::new()
        .allow_origin(frontend_url.parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods(Any)
        .allow_headers(Any);
        
    let security_headers = tower::ServiceBuilder::new()
        .layer(SetResponseHeaderLayer::overriding(
            X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            STRICT_TRANSPORT_SECURITY,
            HeaderValue::from_static("max-age=31536000; includeSubDomains"),
        ));

    let admin_routes = Router::new()
        .route("/users",             get(list_users))
        .route("/users/:id/role",    patch(update_user_role))
        .route("/users/:id/status",  patch(toggle_user_status))
        .route("/tenants",           get(list_tenants).post(create_tenant))
        .route("/tenants/:id",       get(get_tenant).put(update_tenant).delete(delete_tenant))
        .layer(from_fn(require_admin));

    let protected = Router::new()
        .route("/dashboard/stats", get(get_stats))
        .route("/products",                    get(list_products).post(create_product))
        .route("/products/:id",                get(get_product).put(update_product).delete(delete_product))
        .route("/products/barcode/:barcode",   get(get_product_by_barcode))
        .route("/categories",      get(list_categories).post(create_category))
        .route("/categories/:id",  get(get_category).put(update_category).delete(delete_category))
        .route("/suppliers",      get(list_suppliers).post(create_supplier))
        .route("/suppliers/:id",  get(get_supplier).put(update_supplier).delete(delete_supplier))
        .route("/movements",  get(list_movements).post(create_movement))
        .route("/geography/regions/:id/stats",        get(get_region_stats))
        .route("/geography/countries/:id/stats",      get(get_country_stats))
        .route("/geography/cities/:id/dashboard",     get(get_city_dashboard_stats))
        .route("/geography/cities-with-inventory",    get(get_cities_with_inventory))
        .route("/reports/inventory",    get(get_inventory_report))
        .route("/reports/low-stock",     get(get_low_stock_report))
        .route("/reports/movements",     get(get_movements_report))
        .route("/reports/valuation",     get(get_valuation_report))
        .merge(admin_routes)
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware));

    let public = Router::new()
        .route("/health",          get(health))
        .route("/auth/login",      post(login))
        .route("/auth/register",   post(register))
        .route("/ws",              get(ws_handler))
        .route("/geography/regions",                  get(list_regions))
        .route("/geography/regions/:id/countries",    get(list_countries_by_region))
        .route("/geography/countries/:id/cities",     get(list_cities_by_country))
        .route("/geography/cities/:id",               get(get_city));

    let app = Router::new()
        .nest("/api", protected)
        .nest("/api", public)
        .layer(security_headers)
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .layer(DefaultBodyLimit::max(2 * 1024 * 1024))
        .with_state(state);

    let addr = config.server_addr();
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("🚀 VibeStock API running on http://{}", addr);

    axum::serve(listener, app).await?;
    Ok(())
}
