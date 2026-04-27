-- OTP verification codes for email-based registration
CREATE TABLE IF NOT EXISTS otp_requests (
    email       TEXT        PRIMARY KEY,
    otp         TEXT        NOT NULL,          -- 6-digit code (plaintext, ephemeral)
    expires_at  TIMESTAMPTZ NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);
