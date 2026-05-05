-- 010_movements_performed_by_nullable.sql
-- Allow deleting users who have stock movement history.
-- Movements are kept for audit purposes; performed_by becomes NULL when the user is deleted.

-- Drop the RESTRICT FK
ALTER TABLE stock_movements
    DROP CONSTRAINT IF EXISTS stock_movements_performed_by_fkey;

-- Allow NULL (existing rows remain unchanged, their UUIDs still exist)
ALTER TABLE stock_movements
    ALTER COLUMN performed_by DROP NOT NULL;

-- Re-add FK with SET NULL so deletions are allowed
ALTER TABLE stock_movements
    ADD CONSTRAINT stock_movements_performed_by_fkey
    FOREIGN KEY (performed_by) REFERENCES users(id) ON DELETE SET NULL;
