use std::sync::Arc;

use axum::{
    middleware,
    routing::{delete, get, patch, post},
    Router,
};
use tokio::sync::broadcast;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use api::{
    auth::auth_middleware,
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
    // ── Tracing ──────────────────────────────────────────────────────────────
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "api=debug,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // ── Config & DB ───────────────────────────────────────────────────────────
    let config = Config::from_env()?;
    let db     = db::connect(&config.database_url).await?;

    // Run migrations automatically
    sqlx::migrate!("./migrations").run(&db).await?;
    tracing::info!("✅ Migrations applied");

    // ── WebSocket broadcast channel ────────────────────────────────────────────
    let (ws_tx, _) = broadcast::channel::<WsEvent>(128);

    let state: AppState = Arc::new(InnerState { db, config: config.clone(), ws_tx });

    // ── CORS ─────────────────────────────────────────────────────────────────
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // ── Protected routes (require JWT) ────────────────────────────────────────
    let protected = Router::new()
        // Dashboard
        .route("/dashboard/stats", get(get_stats))
        // Products
        .route("/products",                    get(list_products).post(create_product))
        .route("/products/:id",                get(get_product).put(update_product).delete(delete_product))
        .route("/products/barcode/:barcode",   get(get_product_by_barcode))
        // Categories
        .route("/categories",      get(list_categories).post(create_category))
        .route("/categories/:id",  get(get_category).put(update_category).delete(delete_category))
        // Suppliers
        .route("/suppliers",      get(list_suppliers).post(create_supplier))
        .route("/suppliers/:id",  get(get_supplier).put(update_supplier).delete(delete_supplier))
        // Stock Movements
        .route("/movements",  get(list_movements).post(create_movement))
        // Geography (protected - tenant-specific stats)
        .route("/geography/regions/:id/stats",        get(get_region_stats))
        .route("/geography/countries/:id/stats",      get(get_country_stats))
        .route("/geography/cities/:id/dashboard",     get(get_city_dashboard_stats))
        .route("/geography/cities-with-inventory",    get(get_cities_with_inventory))
        // Admin: User Management
        .route("/users",             get(list_users))
        .route("/users/:id/role",    patch(update_user_role))
        .route("/users/:id/status",  patch(toggle_user_status))
        // Tenants (global admin only)
        .route("/tenants",           get(list_tenants).post(create_tenant))
        .route("/tenants/:id",       get(get_tenant).put(update_tenant).delete(delete_tenant))
        // Reports
        .route("/reports/inventory",    get(get_inventory_report))
        .route("/reports/low-stock",     get(get_low_stock_report))
        .route("/reports/movements",     get(get_movements_report))
        .route("/reports/valuation",     get(get_valuation_report))
        // Apply auth middleware
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware));

    // ── Public routes ─────────────────────────────────────────────────────────
    let public = Router::new()
        .route("/health",          get(health))
        .route("/auth/login",      post(login))
        .route("/auth/register",   post(register))
        .route("/ws",              get(ws_handler))
        // Geography (public - read-only global data)
        .route("/geography/regions",                  get(list_regions))
        .route("/geography/regions/:id/countries",    get(list_countries_by_region))
        .route("/geography/countries/:id/cities",     get(list_cities_by_country))
        .route("/geography/cities/:id",               get(get_city));

    // ── Full router ───────────────────────────────────────────────────────────
    let app = Router::new()
        .nest("/api", protected)
        .nest("/api", public)
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state);

    let addr = config.server_addr();
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("🚀 VibeStock API running on http://{}", addr);

    axum::serve(listener, app).await?;
    Ok(())
}
