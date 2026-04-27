use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url:    String,
    pub jwt_secret:      String,
    pub server_host:     String,
    pub server_port:     u16,
    pub jwt_expiry_hours: i64,

    // SMTP — all optional; if smtp_host is None, OTP emails are disabled.
    pub smtp_host: Option<String>,
    pub smtp_port: u16,
    pub smtp_user: Option<String>,
    pub smtp_pass: Option<String>,
    pub smtp_from: Option<String>,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();
        Ok(Config {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/nexstock".into()),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "nexstock_super_secret_jwt_key_mca_2024".into()),
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".into())
                .parse()
                .unwrap_or(3000),
            jwt_expiry_hours: env::var("JWT_EXPIRY_HOURS")
                .unwrap_or_else(|_| "24".into())
                .parse()
                .unwrap_or(24),

            smtp_host: env::var("SMTP_HOST").ok(),
            smtp_port: env::var("SMTP_PORT")
                .unwrap_or_else(|_| "587".into())
                .parse()
                .unwrap_or(587),
            smtp_user: env::var("SMTP_USER").ok(),
            smtp_pass: env::var("SMTP_PASS").ok(),
            smtp_from: env::var("SMTP_FROM").ok(),
        })
    }

    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }

    /// Returns true when all SMTP fields are present.
    pub fn smtp_enabled(&self) -> bool {
        self.smtp_host.is_some()
            && self.smtp_user.is_some()
            && self.smtp_pass.is_some()
            && self.smtp_from.is_some()
    }
}
