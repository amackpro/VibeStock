-- =============================================================================
-- Migration: 004_multi_tenant_support.sql
-- Description: Add multi-tenant support with tenants table and tenant_id columns
-- =============================================================================

-- Enable UUID extension (if not already)
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- =============================================================================
-- TABLE: tenants
-- =============================================================================
CREATE TABLE tenants (
    id              UUID          PRIMARY KEY DEFAULT uuid_generate_v4(),
    name            VARCHAR(255)  NOT NULL,
    slug            VARCHAR(100)  UNIQUE NOT NULL,
    owner_user_id   UUID          REFERENCES users(id) ON DELETE SET NULL,
    is_active       BOOLEAN       NOT NULL DEFAULT true,
    created_at      TIMESTAMPTZ   NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ   NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_tenants_slug ON tenants(slug);
CREATE INDEX idx_tenants_owner ON tenants(owner_user_id);

-- =============================================================================
-- Add tenant_id and is_global_admin to users
-- =============================================================================
ALTER TABLE users ADD COLUMN tenant_id UUID REFERENCES tenants(id) ON DELETE SET NULL;
ALTER TABLE users ADD COLUMN is_global_admin BOOLEAN NOT NULL DEFAULT false;

CREATE INDEX idx_users_tenant ON users(tenant_id);
CREATE INDEX idx_users_global_admin ON users(is_global_admin);

-- =============================================================================
-- Add tenant_id to categories
-- =============================================================================
ALTER TABLE categories ADD COLUMN tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE;

CREATE INDEX idx_categories_tenant ON categories(tenant_id);

-- =============================================================================
-- Add tenant_id to suppliers
-- =============================================================================
ALTER TABLE suppliers ADD COLUMN tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE;

CREATE INDEX idx_suppliers_tenant ON suppliers(tenant_id);

-- =============================================================================
-- Add tenant_id to products
-- =============================================================================
ALTER TABLE products ADD COLUMN tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE;

CREATE INDEX idx_products_tenant ON products(tenant_id);

-- =============================================================================
-- Add tenant_id to stock_movements
-- =============================================================================
ALTER TABLE stock_movements ADD COLUMN tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE;

CREATE INDEX idx_movements_tenant ON stock_movements(tenant_id);

-- =============================================================================
-- Create default tenant for existing data
-- =============================================================================
INSERT INTO tenants (id, name, slug, is_active, created_at, updated_at)
VALUES 
    ('00000000-0000-0000-0000-000000000001', 'Default Tenant', 'default', true, NOW(), NOW()),
    ('00000000-0000-0000-0000-000000000002', 'VibeStock Demo', 'demo', true, NOW(), NOW())
ON CONFLICT (slug) DO NOTHING;

-- =============================================================================
-- Update existing data to use default tenant
-- =============================================================================
UPDATE users SET tenant_id = '00000000-0000-0000-0000-000000000001' WHERE tenant_id IS NULL;
UPDATE categories SET tenant_id = '00000000-0000-0000-0000-000000000001' WHERE tenant_id IS NULL;
UPDATE suppliers SET tenant_id = '00000000-0000-0000-0000-000000000001' WHERE tenant_id IS NULL;
UPDATE products SET tenant_id = '00000000-0000-0000-0000-000000000001' WHERE tenant_id IS NULL;
UPDATE stock_movements SET tenant_id = '00000000-0000-0000-0000-000000000001' WHERE tenant_id IS NULL;

-- Make original admin user a global admin
UPDATE users SET is_global_admin = true WHERE username = 'admin';

-- Set owner for default tenant
UPDATE tenants SET owner_user_id = (SELECT id FROM users WHERE username = 'admin' LIMIT 1)
WHERE id = '00000000-0000-0000-0000-000000000001';

-- =============================================================================
-- Trigger for tenants updated_at
-- =============================================================================
CREATE TRIGGER trg_tenants_updated_at
    BEFORE UPDATE ON tenants
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
