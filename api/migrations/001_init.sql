-- =============================================================================
-- Migration: 001_init.sql
-- Description: Initial schema for VibeStock Inventory Management System
-- Creates: users, categories, suppliers, products, stock_movements tables
-- Author: VibeStock MCA Final Year Project
-- =============================================================================

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- =============================================================================
-- ENUM TYPES
-- =============================================================================

-- User role hierarchy
CREATE TYPE user_role AS ENUM (
    'admin',    -- Full system access
    'manager',  -- Manage inventory, view reports
    'staff'     -- Record stock movements only
);

-- Stock movement direction
CREATE TYPE movement_type AS ENUM (
    'in',           -- Stock received from supplier
    'out',          -- Stock dispatched to customer
    'adjustment',   -- Manual correction (audit, damage, etc.)
    'return'        -- Customer/supplier return
);

-- =============================================================================
-- TABLE: users
-- Stores authenticated system users with role-based access control
-- =============================================================================
CREATE TABLE users (
    id            UUID         PRIMARY KEY DEFAULT uuid_generate_v4(),
    username      VARCHAR(50)  NOT NULL UNIQUE,
    email         VARCHAR(255) NOT NULL UNIQUE,
    password_hash TEXT         NOT NULL,
    full_name     VARCHAR(255) NOT NULL,
    role          user_role    NOT NULL DEFAULT 'staff',
    is_active     BOOLEAN      NOT NULL DEFAULT true,
    created_at    TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email    ON users(email);

-- =============================================================================
-- TABLE: categories
-- Product classification for filtering and reporting
-- =============================================================================
CREATE TABLE categories (
    id          UUID         PRIMARY KEY DEFAULT uuid_generate_v4(),
    name        VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_categories_name ON categories(name);

-- =============================================================================
-- TABLE: suppliers
-- Vendor/supplier master data with contact information
-- =============================================================================
CREATE TABLE suppliers (
    id           UUID         PRIMARY KEY DEFAULT uuid_generate_v4(),
    name         VARCHAR(255) NOT NULL,
    contact_name VARCHAR(255),
    email        VARCHAR(255),
    phone        VARCHAR(50),
    address      TEXT,
    created_at   TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_suppliers_name ON suppliers(name);

-- =============================================================================
-- TABLE: products
-- Core inventory product catalogue with pricing and stock levels
-- =============================================================================
CREATE TABLE products (
    id                UUID          PRIMARY KEY DEFAULT uuid_generate_v4(),
    name              VARCHAR(255)  NOT NULL,
    description       TEXT,
    sku               VARCHAR(100)  NOT NULL UNIQUE,  -- Stock Keeping Unit
    barcode           VARCHAR(100)  UNIQUE,            -- EAN-13 / QR
    category_id       UUID          REFERENCES categories(id) ON DELETE SET NULL,
    supplier_id       UUID          REFERENCES suppliers(id)  ON DELETE SET NULL,
    unit_price        FLOAT8        NOT NULL DEFAULT 0.0 CHECK (unit_price >= 0),
    cost_price        FLOAT8        NOT NULL DEFAULT 0.0 CHECK (cost_price >= 0),
    quantity_in_stock INT           NOT NULL DEFAULT 0 CHECK (quantity_in_stock >= 0),
    reorder_level     INT           NOT NULL DEFAULT 10, -- Alert threshold
    unit_of_measure   VARCHAR(50)   NOT NULL DEFAULT 'pcs',
    is_active         BOOLEAN       NOT NULL DEFAULT true,
    image_url         TEXT,
    created_at        TIMESTAMPTZ   NOT NULL DEFAULT NOW(),
    updated_at        TIMESTAMPTZ   NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_products_sku        ON products(sku);
CREATE INDEX idx_products_barcode    ON products(barcode);
CREATE INDEX idx_products_category   ON products(category_id);
CREATE INDEX idx_products_supplier   ON products(supplier_id);
CREATE INDEX idx_products_is_active  ON products(is_active);
-- Full-text search index for product name and description
CREATE INDEX idx_products_name_trgm  ON products USING gin(to_tsvector('english', name));

-- =============================================================================
-- TABLE: stock_movements
-- Immutable audit log of every stock change — never delete, only insert
-- =============================================================================
CREATE TABLE stock_movements (
    id            UUID          PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id    UUID          NOT NULL REFERENCES products(id) ON DELETE RESTRICT,
    movement_type movement_type NOT NULL,
    quantity      INT           NOT NULL CHECK (quantity > 0),
    reference     VARCHAR(255),  -- PO number, invoice, order ref etc.
    notes         TEXT,
    performed_by  UUID          NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    created_at    TIMESTAMPTZ   NOT NULL DEFAULT NOW()
    -- No updated_at: movements are immutable by design
);

CREATE INDEX idx_movements_product    ON stock_movements(product_id);
CREATE INDEX idx_movements_type       ON stock_movements(movement_type);
CREATE INDEX idx_movements_created_at ON stock_movements(created_at DESC);
CREATE INDEX idx_movements_performed  ON stock_movements(performed_by);

-- =============================================================================
-- FUNCTION: auto-update updated_at timestamp
-- =============================================================================
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Attach trigger to all tables with updated_at
CREATE TRIGGER trg_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER trg_categories_updated_at
    BEFORE UPDATE ON categories
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER trg_suppliers_updated_at
    BEFORE UPDATE ON suppliers
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER trg_products_updated_at
    BEFORE UPDATE ON products
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
