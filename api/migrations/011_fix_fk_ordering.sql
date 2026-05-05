-- 011_fix_fk_ordering.sql
-- Migration 009 could fail when orphaned users have stock movements because
-- the old ON DELETE RESTRICT FK blocks DELETE FROM users.
-- This migration applies all fixes in the correct order and is fully idempotent.

-- Step 1: NULL out performed_by for any movements made by orphaned users
--         (users with no tenant and not a global admin)
UPDATE stock_movements
SET performed_by = NULL
WHERE performed_by IN (
    SELECT id FROM users WHERE tenant_id IS NULL AND is_global_admin = false
);

-- Step 2: Now it is safe to delete orphaned users (no FK blocking)
DELETE FROM users WHERE tenant_id IS NULL AND is_global_admin = false;

-- Step 3: Fix the users → tenants FK to CASCADE on tenant delete (idempotent)
ALTER TABLE users DROP CONSTRAINT IF EXISTS users_tenant_id_fkey;
ALTER TABLE users
    ADD CONSTRAINT users_tenant_id_fkey
    FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE;

-- Step 4: Fix stock_movements → users FK to SET NULL on user delete (idempotent)
ALTER TABLE stock_movements DROP CONSTRAINT IF EXISTS stock_movements_performed_by_fkey;
ALTER TABLE stock_movements ALTER COLUMN performed_by DROP NOT NULL;
ALTER TABLE stock_movements
    ADD CONSTRAINT stock_movements_performed_by_fkey
    FOREIGN KEY (performed_by) REFERENCES users(id) ON DELETE SET NULL;
