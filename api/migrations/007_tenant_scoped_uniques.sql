-- Migration: 007_tenant_scoped_uniques.sql
-- Description: Change global unique constraints to tenant-scoped unique constraints
-- This allows different tenants to have products with the same SKU/barcode and categories with the same name.

-- 1. PRODUCTS TABLE
-- Remove global unique constraints
ALTER TABLE products DROP CONSTRAINT IF EXISTS products_sku_key;
ALTER TABLE products DROP CONSTRAINT IF EXISTS products_barcode_key;

-- Add tenant-scoped unique constraints
-- Note: barcode can be NULL, so we use a standard unique index/constraint which allows multiple NULLs but unique values per tenant
ALTER TABLE products ADD CONSTRAINT products_tenant_sku_key UNIQUE (tenant_id, sku);
ALTER TABLE products ADD CONSTRAINT products_tenant_barcode_key UNIQUE (tenant_id, barcode);

-- 2. CATEGORIES TABLE
-- Remove global unique constraint
ALTER TABLE categories DROP CONSTRAINT IF EXISTS categories_name_key;

-- Add tenant-scoped unique constraint
ALTER TABLE categories ADD CONSTRAINT categories_tenant_name_key UNIQUE (tenant_id, name);

-- 3. USERS TABLE
-- Username and email should generally remain globally unique in this system's design (for login),
-- but if we wanted per-tenant usernames, we would change it here. 
-- For now, we'll keep users globally unique to avoid login ambiguity.
