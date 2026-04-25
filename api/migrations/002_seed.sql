-- =============================================================================
-- Migration: 002_seed.sql
-- Description: Minimal UI Showcase Data (Reset)
-- =============================================================================

-- GEOGRAPHY: Regions, Countries, Cities
-- Asia Region
INSERT INTO regions (id, name, code, latitude, longitude) VALUES
('rr000000-0000-0000-0000-000000000001', 'Asia', 'AS', 34.0, 100.0)
ON CONFLICT (name) DO NOTHING;

-- India
INSERT INTO countries (id, region_id, name, iso2, iso3, phone_code, capital, latitude, longitude) VALUES
('cc000000-0000-0000-0000-000000000001', 'rr000000-0000-0000-0000-000000000001', 'India', 'IN', 'IND', '+91', 'New Delhi', 20.5937, 78.9629)
ON CONFLICT (iso2) DO NOTHING;

-- Indian Cities
INSERT INTO cities (id, country_id, name, state_name, latitude, longitude, population, is_capital) VALUES
('ci000000-0000-0000-0000-000000000001', 'cc000000-0000-0000-0000-000000000001', 'Mumbai', 'Maharashtra', 19.0760, 72.8777, 20411000, true),
('ci000000-0000-0000-0000-000000000002', 'cc000000-0000-0000-0000-000000000001', 'Delhi', 'Delhi', 28.7041, 77.1025, 31181000, true),
('ci000000-0000-0000-0000-000000000003', 'cc000000-0000-0000-0000-000000000001', 'Bangalore', 'Karnataka', 12.9716, 77.5946, 12327000, false),
('ci000000-0000-0000-0000-000000000004', 'cc000000-0000-0000-0000-000000000001', 'Kolkata', 'West Bengal', 22.5726, 88.3639, 14850000, false),
('ci000000-0000-0000-0000-000000000005', 'cc000000-0000-0000-0000-000000000001', 'Chennai', 'Tamil Nadu', 13.0827, 80.2707, 10971000, false),
('ci000000-0000-0000-0000-000000000006', 'cc000000-0000-0000-0000-000000000001', 'Hyderabad', 'Telangana', 17.3850, 78.4867, 10534000, false),
('ci000000-0000-0000-0000-000000000007', 'cc000000-0000-0000-0000-000000000001', 'Pune', 'Maharashtra', 18.5204, 73.8567, 6276000, false),
('ci000000-0000-0000-0000-000000000008', 'cc000000-0000-0000-0000-000000000001', 'Ahmedabad', 'Gujarat', 23.0225, 72.5714, 8253000, false)
ON CONFLICT (name) DO NOTHING;

-- USERS (Password: Password@123)
INSERT INTO users (id, username, email, password_hash, full_name, role) VALUES
('aa000000-0000-0000-0000-000000000001', 'admin', 'admin@vibestock.local', '$2b$12$49a3Bj648XXrBBWJrM87TuG/zHT8JPW1Kvx1QUvQXXCz5hoQQn23u', 'System Administrator', 'admin'),
('aa000000-0000-0000-0000-000000000002', 'manager', 'manager@vibestock.local', '$2b$12$49a3Bj648XXrBBWJrM87TuG/zHT8JPW1Kvx1QUvQXXCz5hoQQn23u', 'Warehouse Manager', 'manager');

-- CATEGORIES
INSERT INTO categories (id, name, description) VALUES
('cc000000-0000-0000-0000-000000000001', 'Electronics', 'Phones, Laptops, Gadgets'),
('cc000000-0000-0000-0000-000000000002', 'Stationery', 'Office supplies');

-- SUPPLIERS
INSERT INTO suppliers (id, name, contact_name, email, phone, address) VALUES
('ee000000-0000-0000-0000-000000000001', 'Global Tech Supplies', 'Alice Smith', 'alice@globaltech.com', '+1-555-1234', '123 Tech Park');

-- PRODUCTS
INSERT INTO products
    (id, name, description, sku, barcode, category_id, supplier_id, unit_price, cost_price, quantity_in_stock, reorder_level, unit_of_measure)
VALUES
('bb000001-0000-0000-0000-000000000001', 'MacBook Pro 16', 'M3 Max, 32GB RAM, 1TB SSD', 'ELEC-MAC-001', '8901234567890', 'cc000000-0000-0000-0000-000000000001', 'ee000000-0000-0000-0000-000000000001', 2500.00, 2000.00, 50, 10, 'pcs'),
('bb000001-0000-0000-0000-000000000002', 'iPhone 15 Pro', '256GB, Titanium', 'ELEC-IPH-002', '8901234567891', 'cc000000-0000-0000-0000-000000000001', 'ee000000-0000-0000-0000-000000000001', 999.00, 850.00, 120, 20, 'pcs'),
('bb000003-0000-0000-0000-000000000001', 'Premium Printer Paper', 'A4 500 Sheets', 'STAT-PPR-001', '8903456789012', 'cc000000-0000-0000-0000-000000000002', 'ee000000-0000-0000-0000-000000000001', 15.00, 8.00, 500, 50, 'ream');

-- STOCK MOVEMENTS
INSERT INTO stock_movements (product_id, movement_type, quantity, reference, notes, performed_by) VALUES
('bb000001-0000-0000-0000-000000000001', 'in', 50, 'PO-2024-001', 'Initial Stock', 'aa000000-0000-0000-0000-000000000001'),
('bb000001-0000-0000-0000-000000000002', 'in', 120, 'PO-2024-001', 'Initial Stock', 'aa000000-0000-0000-0000-000000000001'),
('bb000003-0000-0000-0000-000000000001', 'in', 500, 'PO-2024-001', 'Initial Stock', 'aa000000-0000-0000-0000-000000000001'),
('bb000001-0000-0000-0000-000000000001', 'out', 2, 'SO-2024-001', 'Sales Order to Client X', 'aa000000-0000-0000-0000-000000000002');
