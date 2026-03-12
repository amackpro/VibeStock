-- =============================================================================
-- Migration: 002_seed.sql
-- Description: Seed data for VibeStock development & demo environment
-- Inserts: admin user, 5 categories, 5 suppliers, 20 products, stock movements
-- =============================================================================

-- =============================================================================
-- USERS  (passwords are bcrypt hash of "Password@123")
-- =============================================================================
INSERT INTO users (id, username, email, password_hash, full_name, role) VALUES
(
    'a0000000-0000-0000-0000-000000000001',
    'admin',
    'admin@vibestock.local',
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LOwna5PPSmlvEKRCy', -- Password@123
    'System Administrator',
    'admin'
),
(
    'a0000000-0000-0000-0000-000000000002',
    'manager',
    'manager@vibestock.local',
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LOwna5PPSmlvEKRCy',
    'Warehouse Manager',
    'manager'
),
(
    'a0000000-0000-0000-0000-000000000003',
    'staff1',
    'staff@vibestock.local',
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LOwna5PPSmlvEKRCy',
    'Stock Staff Member',
    'staff'
);

-- =============================================================================
-- CATEGORIES
-- =============================================================================
INSERT INTO categories (id, name, description) VALUES
('c0000000-0000-0000-0000-000000000001', 'Electronics',   'Electronic devices, components, and accessories'),
('c0000000-0000-0000-0000-000000000002', 'Furniture',      'Office and warehouse furniture'),
('c0000000-0000-0000-0000-000000000003', 'Stationery',     'Office stationery and paper products'),
('c0000000-0000-0000-0000-000000000004', 'Packaging',      'Boxes, tape, bubble wrap and shipping materials'),
('c0000000-0000-0000-0000-000000000005', 'Safety & Tools', 'Safety equipment and workshop tools');

-- =============================================================================
-- SUPPLIERS
-- =============================================================================
INSERT INTO suppliers (id, name, contact_name, email, phone, address) VALUES
(
    's0000000-0000-0000-0000-000000000001',
    'TechTrade India Ltd.',
    'Rajesh Kumar',
    'rajesh@techtrade.in',
    '+91-9876543210',
    '42, Electronics Complex, Nehru Place, New Delhi - 110019'
),
(
    's0000000-0000-0000-0000-000000000002',
    'Furniture World Pvt. Ltd.',
    'Priya Sharma',
    'priya@furnitureworld.co.in',
    '+91-9765432109',
    '7, Industrial Area Phase II, Chandigarh - 160002'
),
(
    's0000000-0000-0000-0000-000000000003',
    'Office Essentials Co.',
    'Ankit Patel',
    'ankit@officeessentials.in',
    '+91-9654321098',
    '15, Nariman Point, Mumbai - 400021'
),
(
    's0000000-0000-0000-0000-000000000004',
    'PackRight Solutions',
    'Deepika Nair',
    'deepika@packright.in',
    '+91-9543210987',
    '88, GIDC Industrial Estate, Ahmedabad - 382210'
),
(
    's0000000-0000-0000-0000-000000000005',
    'SafeGuard Supplies',
    'Vikram Singh',
    'vikram@safeguard.in',
    '+91-9432109876',
    '23, MG Road, Bengaluru - 560001'
);

-- =============================================================================
-- PRODUCTS (20 items across all categories)
-- =============================================================================
INSERT INTO products
    (id, name, description, sku, barcode, category_id, supplier_id,
     unit_price, cost_price, quantity_in_stock, reorder_level, unit_of_measure)
VALUES
-- Electronics (6)
('p0000001-0000-0000-0000-000000000001', 'Laptop Dell Inspiron 15',
 '15.6" FHD, Intel i5-12th Gen, 8GB RAM, 512GB SSD', 'ELEC-LAP-001', '8901234567890',
 'c0000000-0000-0000-0000-000000000001', 's0000000-0000-0000-0000-000000000001',
 55000.00, 48000.00, 25, 5, 'pcs'),

('p0000001-0000-0000-0000-000000000002', 'Wireless Mouse Logitech M705',
 'Silent wireless mouse, 3-year battery life', 'ELEC-MSE-002', '8901234567891',
 'c0000000-0000-0000-0000-000000000001', 's0000000-0000-0000-0000-000000000001',
 2500.00, 1800.00, 80, 20, 'pcs'),

('p0000001-0000-0000-0000-000000000003', 'USB-C Hub 7-in-1',
 'HDMI, USB-A, SD card, PD charging', 'ELEC-HUB-003', '8901234567892',
 'c0000000-0000-0000-0000-000000000001', 's0000000-0000-0000-0000-000000000001',
 1800.00, 1200.00, 45, 15, 'pcs'),

('p0000001-0000-0000-0000-000000000004', 'Monitor 24" Full HD',
 'IPS panel, 75Hz, HDMI+VGA, VESA mount', 'ELEC-MON-004', '8901234567893',
 'c0000000-0000-0000-0000-000000000001', 's0000000-0000-0000-0000-000000000001',
 14000.00, 11500.00, 12, 3, 'pcs'),

('p0000001-0000-0000-0000-000000000005', 'Mechanical Keyboard Redragon',
 'TKL layout, Blue switches, RGB backlight', 'ELEC-KBD-005', '8901234567894',
 'c0000000-0000-0000-0000-000000000001', 's0000000-0000-0000-0000-000000000001',
 3200.00, 2400.00, 35, 10, 'pcs'),

('p0000001-0000-0000-0000-000000000006', 'Webcam Logitech C920',
 '1080p HD, built-in stereo mic, autofocus', 'ELEC-CAM-006', '8901234567895',
 'c0000000-0000-0000-0000-000000000001', 's0000000-0000-0000-0000-000000000001',
 6500.00, 5000.00, 3, 5, 'pcs'),  -- Below reorder level (demo)

-- Furniture (4)
('p0000002-0000-0000-0000-000000000001', 'Ergonomic Office Chair',
 'Mesh back, lumbar support, adjustable armrests', 'FURN-CHR-001', '8902345678901',
 'c0000000-0000-0000-0000-000000000002', 's0000000-0000-0000-0000-000000000002',
 12000.00, 9000.00, 18, 5, 'pcs'),

('p0000002-0000-0000-0000-000000000002', 'Standing Desk 140cm',
 'Height adjustable, electric motor, memory settings', 'FURN-DSK-002', '8902345678902',
 'c0000000-0000-0000-0000-000000000002', 's0000000-0000-0000-0000-000000000002',
 22000.00, 17000.00, 8, 3, 'pcs'),

('p0000002-0000-0000-0000-000000000003', 'Filing Cabinet 4-Drawer',
 'Steel construction, lockable, A4 size', 'FURN-CAB-003', '8902345678903',
 'c0000000-0000-0000-0000-000000000002', 's0000000-0000-0000-0000-000000000002',
 8500.00, 6500.00, 0, 2, 'pcs'),  -- Out of stock (demo)

('p0000002-0000-0000-0000-000000000004', 'Bookshelf 5-Tier',
 'Particleboard, walnut finish, 180cm height', 'FURN-BSH-004', '8902345678904',
 'c0000000-0000-0000-0000-000000000002', 's0000000-0000-0000-0000-000000000002',
 4200.00, 3000.00, 22, 5, 'pcs'),

-- Stationery (4)
('p0000003-0000-0000-0000-000000000001', 'A4 Copier Paper 500 Sheets',
 '80 GSM, ream of 500, suitable for all printers', 'STAT-PPR-001', '8903456789012',
 'c0000000-0000-0000-0000-000000000003', 's0000000-0000-0000-0000-000000000003',
 420.00, 320.00, 200, 50, 'ream'),

('p0000003-0000-0000-0000-000000000002', 'Ballpoint Pen Box (50 pcs)',
 'Blue ink, smooth writing, medium tip', 'STAT-PEN-002', '8903456789013',
 'c0000000-0000-0000-0000-000000000003', 's0000000-0000-0000-0000-000000000003',
 180.00, 120.00, 150, 30, 'box'),

('p0000003-0000-0000-0000-000000000003', 'Sticky Notes 3x3" (12 pads)',
 'Neon colours, 100 sheets per pad', 'STAT-STK-003', '8903456789014',
 'c0000000-0000-0000-0000-000000000003', 's0000000-0000-0000-0000-000000000003',
 350.00, 240.00, 75, 20, 'pack'),

('p0000003-0000-0000-0000-000000000004', 'Whiteboard Marker Set (8 colours)',
 'Dry-erase, chisel tip, easy clean', 'STAT-MRK-004', '8903456789015',
 'c0000000-0000-0000-0000-000000000003', 's0000000-0000-0000-0000-000000000003',
 220.00, 150.00, 4, 10, 'set'),  -- Below reorder level (demo)

-- Packaging (3)
('p0000004-0000-0000-0000-000000000001', 'Cardboard Box 30x20x15cm',
 'Single wall, brown kraft, pack of 25', 'PACK-BOX-001', '8904567890123',
 'c0000000-0000-0000-0000-000000000004', 's0000000-0000-0000-0000-000000000004',
 850.00, 600.00, 500, 100, 'pack'),

('p0000004-0000-0000-0000-000000000002', 'Bubble Wrap Roll 50m',
 '10mm bubble, 500mm wide, protective packaging', 'PACK-BWR-002', '8904567890124',
 'c0000000-0000-0000-0000-000000000004', 's0000000-0000-0000-0000-000000000004',
 1200.00, 900.00, 30, 10, 'roll'),

('p0000004-0000-0000-0000-000000000003', 'Brown Packing Tape 48mm x 50m',
 'Hot melt adhesive, strong bond, pack of 6', 'PACK-TPE-003', '8904567890125',
 'c0000000-0000-0000-0000-000000000004', 's0000000-0000-0000-0000-000000000004',
 480.00, 340.00, 120, 30, 'pack'),

-- Safety & Tools (3)
('p0000005-0000-0000-0000-000000000001', 'Safety Helmet EN397',
 'HDPE shell, adjustable ratchet, ventilated', 'SAFE-HLM-001', '8905678901234',
 'c0000000-0000-0000-0000-000000000005', 's0000000-0000-0000-0000-000000000005',
 750.00, 520.00, 40, 10, 'pcs'),

('p0000005-0000-0000-0000-000000000002', 'Safety Gloves Cut-Resistant L',
 'Level 5 cut protection, HPPE liner, grip coating', 'SAFE-GLV-002', '8905678901235',
 'c0000000-0000-0000-0000-000000000005', 's0000000-0000-0000-0000-000000000005',
 380.00, 260.00, 60, 20, 'pair'),

('p0000005-0000-0000-0000-000000000003', 'Torque Wrench 1/2" Drive',
 '20-200 Nm range, click-type, chrome vanadium', 'SAFE-TQW-003', '8905678901236',
 'c0000000-0000-0000-0000-000000000005', 's0000000-0000-0000-0000-000000000005',
 2800.00, 2000.00, 15, 4, 'pcs');

-- =============================================================================
-- STOCK MOVEMENTS (initial stock-in movements for all products)
-- =============================================================================
INSERT INTO stock_movements
    (product_id, movement_type, quantity, reference, notes, performed_by)
SELECT
    id,
    'in'::movement_type,
    quantity_in_stock + 10,    -- Original received qty before any out movements
    'PO-INIT-2024',
    'Initial stock loading on system setup',
    'a0000000-0000-0000-0000-000000000001'  -- admin
FROM products
WHERE quantity_in_stock > 0;

-- A few sample OUT movements to show history
INSERT INTO stock_movements
    (product_id, movement_type, quantity, reference, notes, performed_by)
VALUES
('p0000001-0000-0000-0000-000000000002', 'out', 5,  'SO-1001', 'Sales order to Acme Corp',      'a0000000-0000-0000-0000-000000000002'),
('p0000001-0000-0000-0000-000000000003', 'out', 3,  'SO-1002', 'Internal IT department request', 'a0000000-0000-0000-0000-000000000003'),
('p0000003-0000-0000-0000-000000000001', 'out', 20, 'SO-1003', 'Monthly office supplies issue',  'a0000000-0000-0000-0000-000000000002'),
('p0000001-0000-0000-0000-000000000001', 'out', 2,  'SO-1004', 'New employee workstation setup', 'a0000000-0000-0000-0000-000000000002'),
('p0000004-0000-0000-0000-000000000001', 'out', 50, 'SO-1005', 'Dispatch to warehouse B',        'a0000000-0000-0000-0000-000000000003'),
('p0000003-0000-0000-0000-000000000004', 'out', 6,  'SO-1006', 'Conference room restock',        'a0000000-0000-0000-0000-000000000003');

-- Adjustment example (damaged goods)
INSERT INTO stock_movements
    (product_id, movement_type, quantity, reference, notes, performed_by)
VALUES
('p0000001-0000-0000-0000-000000000006', 'adjustment', 3, 'ADJ-0001',
 'Damaged units during transit — written off after physical audit',
 'a0000000-0000-0000-0000-000000000001');
