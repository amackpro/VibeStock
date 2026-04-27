use axum::{extract::State, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use lettre::{
    message::header::ContentType,
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use rand::Rng;
use uuid::Uuid;
use validator::Validate;

use crate::{auth::create_token, AppState};
use shared::{AppError, AppResult, LoginRequest, LoginResponse, RegisterRequest, SendOtpRequest, VerifyOtpRequest, TenantInfo};

// ─── Login ────────────────────────────────────────────────────────────────────

/// POST /api/auth/login
/// Verifies credentials and returns a signed JWT with tenant info.
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<Json<LoginResponse>> {

    #[derive(sqlx::FromRow)]
    struct LoginRow {
        id:               Uuid,
        tenant_id:        Uuid,
        is_global_admin:  bool,
        username:         String,
        password_hash:    String,
        full_name:        String,
        role:             Option<String>,
        tenant_name:      Option<String>,
    }

    let row = sqlx::query_as::<_, LoginRow>(
        "SELECT u.id, u.tenant_id, u.is_global_admin, u.username, u.password_hash,
                u.full_name, u.role::text AS role, t.name as tenant_name
         FROM users u
         LEFT JOIN tenants t ON t.id = u.tenant_id
         WHERE u.username = $1 AND u.is_active = true"
    )
    .bind(&payload.username)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::Unauthorized("Invalid username or password".into()))?;

    let valid = verify(&payload.password, &row.password_hash)
        .map_err(|_| AppError::Internal("Password verification failed".into()))?;

    if !valid {
        return Err(AppError::Unauthorized("Invalid username or password".into()));
    }

    let role_str    = row.role.unwrap_or_else(|| "staff".into());
    let tenant_name = row.tenant_name.unwrap_or_else(|| "Unknown".to_string());

    let token = create_token(
        row.id,
        &row.username,
        &role_str,
        row.tenant_id,
        row.is_global_admin,
        &state.config.jwt_secret,
        state.config.jwt_expiry_hours,
    )?;

    let accessible_tenants = if row.is_global_admin {
        sqlx::query_as::<_, TenantInfo>(
            "SELECT id, name, slug FROM tenants WHERE is_active = true ORDER BY name"
        )
        .fetch_all(&state.db)
        .await?
    } else {
        vec![TenantInfo {
            id:   row.tenant_id,
            name: tenant_name.clone(),
            slug: "".to_string(),
        }]
    };

    Ok(Json(LoginResponse {
        token,
        user_id:           row.id,
        username:          row.username,
        full_name:         row.full_name,
        role:              role_str,
        tenant_id:         row.tenant_id,
        tenant_name,
        is_global_admin:   row.is_global_admin,
        accessible_tenants,
    }))
}

// ─── Send OTP ─────────────────────────────────────────────────────────────────

/// POST /api/auth/send-otp
/// Generates a 6-digit OTP, stores it in otp_requests, and emails it.
pub async fn send_otp(
    State(state): State<AppState>,
    Json(payload): Json<SendOtpRequest>,
) -> AppResult<Json<serde_json::Value>> {
    if let Err(e) = payload.validate() {
        return Err(AppError::BadRequest(format!("Validation error: {}", e)));
    }

    if !state.config.smtp_enabled() {
        return Err(AppError::Internal(
            "Email service is not configured on this server.".into(),
        ));
    }

    // Generate zero-padded 6-digit OTP
    let otp_code = format!("{:06}", rand::thread_rng().gen_range(0..1_000_000u32));

    // Store in DB with 10-minute expiry (upsert — allows resend)
    let expires_at = Utc::now() + chrono::Duration::minutes(10);
    sqlx::query(
        "INSERT INTO otp_requests (email, otp, expires_at) \
         VALUES ($1, $2, $3) \
         ON CONFLICT (email) DO UPDATE \
         SET otp = $2, expires_at = $3, created_at = now()"
    )
    .bind(&payload.email)
    .bind(&otp_code)
    .bind(expires_at)
    .execute(&state.db)
    .await?;

    // Build email
    let smtp_from = state.config.smtp_from.as_deref().unwrap();
    let from_addr = format!("NexStock <{}>", smtp_from)
        .parse()
        .map_err(|_| AppError::Internal("Invalid SMTP_FROM address".into()))?;
    let to_addr = payload.email
        .parse()
        .map_err(|_| AppError::BadRequest("Invalid email address".into()))?;

    let body = format!(
        r#"<!DOCTYPE html>
<html>
<body style="font-family: Arial, sans-serif; background: #f5f5f5; padding: 30px;">
  <div style="max-width: 480px; margin: 0 auto; background: #fff; border-radius: 12px; padding: 32px; box-shadow: 0 2px 12px rgba(0,0,0,0.08);">
    <h2 style="color: #6366f1; margin-top: 0;">NexStock — Email Verification</h2>
    <p style="color: #444;">Use the OTP below to complete your registration:</p>
    <div style="font-size: 40px; font-weight: bold; letter-spacing: 12px; color: #6366f1;
                background: #f0f0ff; border-radius: 10px; text-align: center; padding: 20px 10px;
                margin: 24px 0;">
      {}
    </div>
    <p style="color: #666;">This code is valid for <strong>10 minutes</strong>.</p>
    <p style="color: #999; font-size: 12px; margin-bottom: 0;">
      If you did not request this, please ignore this email.
    </p>
  </div>
</body>
</html>"#,
        otp_code
    );

    let email = Message::builder()
        .from(from_addr)
        .to(to_addr)
        .subject("Your NexStock Verification Code")
        .header(ContentType::TEXT_HTML)
        .body(body)
        .map_err(|e| AppError::Internal(format!("Email build error: {}", e)))?;

    // Create SMTP transport (STARTTLS on port 587)
    let smtp_host = state.config.smtp_host.as_deref().unwrap();
    let creds = Credentials::new(
        state.config.smtp_user.clone().unwrap(),
        state.config.smtp_pass.clone().unwrap(),
    );

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(smtp_host)
        .map_err(|e| AppError::Internal(format!("SMTP config error: {}", e)))?
        .port(state.config.smtp_port)
        .credentials(creds)
        .build();

    mailer
        .send(email)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to send email: {}", e)))?;

    Ok(Json(serde_json::json!({
        "message": "OTP sent to your email. Please check your inbox."
    })))
}

// ─── Verify OTP (instant check, does not consume) ────────────────────────────

/// POST /api/auth/verify-otp
/// Checks whether the OTP is correct without consuming it.
/// Used for real-time feedback as the user types.
pub async fn verify_otp(
    State(state): State<AppState>,
    Json(payload): Json<VerifyOtpRequest>,
) -> AppResult<Json<serde_json::Value>> {
    #[derive(sqlx::FromRow)]
    struct OtpRow {
        otp:        String,
        expires_at: chrono::DateTime<Utc>,
    }

    let record = sqlx::query_as::<_, OtpRow>(
        "SELECT otp, expires_at FROM otp_requests WHERE email = $1"
    )
    .bind(&payload.email)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::BadRequest("No OTP found for this email.".into()))?;

    if Utc::now() > record.expires_at {
        return Err(AppError::BadRequest("OTP has expired. Please request a new one.".into()));
    }

    if payload.otp.trim() != record.otp {
        return Err(AppError::Unauthorized("Incorrect OTP.".into()));
    }

    Ok(Json(serde_json::json!({ "valid": true })))
}

// ─── Register ─────────────────────────────────────────────────────────────────

/// POST /api/auth/register
/// Creates a new user account. Verifies OTP first if SMTP is configured.
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> AppResult<Json<serde_json::Value>> {
    if let Err(e) = payload.validate() {
        return Err(AppError::BadRequest(format!("Validation error: {}", e)));
    }

    // ── OTP verification (when SMTP is enabled) ───────────────────────────────
    if state.config.smtp_enabled() {
        let otp_provided = payload.otp.as_deref().unwrap_or("").trim().to_string();
        if otp_provided.is_empty() {
            return Err(AppError::BadRequest(
                "Email verification is required. Please verify your email with the OTP sent to it.".into(),
            ));
        }

        #[derive(sqlx::FromRow)]
        struct OtpRow {
            otp:        String,
            expires_at: chrono::DateTime<Utc>,
        }

        let record = sqlx::query_as::<_, OtpRow>(
            "SELECT otp, expires_at FROM otp_requests WHERE email = $1"
        )
        .bind(&payload.email)
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| AppError::BadRequest(
            "No OTP found for this email. Please request a new one.".into(),
        ))?;

        if Utc::now() > record.expires_at {
            // Clean up expired entry
            sqlx::query("DELETE FROM otp_requests WHERE email = $1")
                .bind(&payload.email)
                .execute(&state.db)
                .await
                .ok();
            return Err(AppError::BadRequest(
                "OTP has expired. Please request a new one.".into(),
            ));
        }

        if otp_provided != record.otp {
            return Err(AppError::Unauthorized("Incorrect OTP. Please try again.".into()));
        }

        // OTP is valid — delete it (one-time use)
        sqlx::query("DELETE FROM otp_requests WHERE email = $1")
            .bind(&payload.email)
            .execute(&state.db)
            .await
            .ok();
    }

    // ── Duplicate check ───────────────────────────────────────────────────────
    let exists: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM users WHERE username = $1 OR email = $2"
    )
    .bind(&payload.username)
    .bind(&payload.email)
    .fetch_one(&state.db)
    .await?;

    if exists > 0 {
        return Err(AppError::Conflict("Username or email already exists".into()));
    }

    let mode = payload.mode.as_deref().unwrap_or("new");

    let (tenant_id, role_to_assign, is_active) = match mode {
        // ── JOIN EXISTING ─────────────────────────────────────────────────────
        "join" => {
            let slug = payload.tenant_slug.as_deref().ok_or_else(|| {
                AppError::BadRequest("tenant_slug is required when joining an organization.".into())
            })?;

            #[derive(sqlx::FromRow)]
            struct TenantRow { id: Uuid }

            let tenant = sqlx::query_as::<_, TenantRow>(
                "SELECT id FROM tenants WHERE slug = $1 AND is_active = true"
            )
            .bind(slug)
            .fetch_optional(&state.db)
            .await?
            .ok_or_else(|| AppError::NotFound(
                "No active organization found with that name. Check the name and try again.".into()
            ))?;

            // Join as inactive staff — org admin must activate
            (tenant.id, "staff", false)
        }

        // ── CREATE NEW ────────────────────────────────────────────────────────
        _ => {
            let tenant_name = payload.tenant_name.as_deref().ok_or_else(|| {
                AppError::BadRequest("Organization name is required when creating a new account.".into())
            })?;

            let slug = tenant_name.to_lowercase().trim().replace(' ', "-");

            let slug_taken: bool = sqlx::query_scalar(
                "SELECT EXISTS(SELECT 1 FROM tenants WHERE slug = $1)"
            )
            .bind(&slug)
            .fetch_one(&state.db)
            .await?;

            if slug_taken {
                return Err(AppError::Conflict(
                    "An organization with this name already exists. \
                     Try joining it instead, or choose a different name.".into(),
                ));
            }

            let tid = Uuid::new_v4();
            sqlx::query("INSERT INTO tenants (id, name, slug) VALUES ($1, $2, $3)")
                .bind(tid)
                .bind(tenant_name)
                .bind(&slug)
                .execute(&state.db)
                .await?;

            (tid, "admin", true)
        }
    };

    let password_hash = hash(&payload.password, DEFAULT_COST)
        .map_err(|e| AppError::Internal(format!("Hashing failed: {}", e)))?;

    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO users (id, tenant_id, username, email, password_hash, full_name, role, is_active) \
         VALUES ($1, $2, $3, $4, $5, $6, $7::user_role, $8)"
    )
    .bind(id)
    .bind(tenant_id)
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(password_hash)
    .bind(&payload.full_name)
    .bind(role_to_assign)
    .bind(is_active)
    .execute(&state.db)
    .await?;

    // Make the founder the org owner
    if role_to_assign == "admin" {
        sqlx::query("UPDATE tenants SET owner_user_id = $1 WHERE id = $2")
            .bind(id)
            .bind(tenant_id)
            .execute(&state.db)
            .await
            .ok();
    }

    let message = if mode == "join" {
        "Join request submitted! Your account is pending activation by the organization admin."
    } else {
        "Organization created successfully! You can now log in."
    };

    Ok(Json(serde_json::json!({
        "message": message,
        "user_id": id,
        "tenant_id": tenant_id,
        "pending_activation": mode == "join"
    })))
}

// ─── Public helpers ───────────────────────────────────────────────────────────

/// GET /api/auth/orgs
/// Public endpoint — returns active orgs for the "join existing" registration dropdown.
pub async fn list_orgs(
    State(state): State<AppState>,
) -> AppResult<Json<Vec<shared::TenantInfo>>> {
    let orgs = sqlx::query_as::<_, shared::TenantInfo>(
        "SELECT id, name, slug FROM tenants WHERE is_active = true ORDER BY name"
    )
    .fetch_all(&state.db)
    .await?;
    Ok(Json(orgs))
}

/// GET /api/health
pub async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status":  "ok",
        "service": "NexStock API",
        "version": "0.1.0"
    }))
}
