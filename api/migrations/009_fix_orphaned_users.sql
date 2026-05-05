-- 009_fix_orphaned_users.sql
-- Clean up users whose tenant was deleted (tenant_id IS NULL, not global admin)
-- and change the FK to ON DELETE CASCADE so future tenant deletes clean up automatically.

DELETE FROM users WHERE tenant_id IS NULL AND is_global_admin = false;

ALTER TABLE users DROP CONSTRAINT IF EXISTS users_tenant_id_fkey;

ALTER TABLE users
    ADD CONSTRAINT users_tenant_id_fkey
    FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE;
