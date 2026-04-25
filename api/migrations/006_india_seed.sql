-- Seed India geography data for suppliers (PostgreSQL compatible)
INSERT INTO regions (id, name, code, latitude, longitude) VALUES
(gen_random_uuid(), 'Asia', 'AS', 34.0, 100.0)
ON CONFLICT (name) DO NOTHING;

INSERT INTO countries (id, region_id, name, iso2, iso3, phone_code, capital, latitude, longitude) 
SELECT gen_random_uuid(), r.id, 'India', 'IN', 'IND', '+91', 'New Delhi', 20.5937, 78.9629
FROM regions r WHERE r.name = 'Asia'
ON CONFLICT (iso2) DO NOTHING;

INSERT INTO cities (id, country_id, name, state_name, latitude, longitude, population, is_capital) 
SELECT gen_random_uuid(), c.id, 'Mumbai', 'Maharashtra', 19.0760, 72.8777, 20411000, true
FROM countries c WHERE c.iso2 = 'IN'
ON CONFLICT (name, country_id) DO NOTHING;

INSERT INTO cities (id, country_id, name, state_name, latitude, longitude, population, is_capital) 
SELECT gen_random_uuid(), c.id, 'Delhi', 'Delhi', 28.7041, 77.1025, 31181000, true
FROM countries c WHERE c.iso2 = 'IN'
ON CONFLICT (name, country_id) DO NOTHING;

INSERT INTO cities (id, country_id, name, state_name, latitude, longitude, population, is_capital) 
SELECT gen_random_uuid(), c.id, 'Bangalore', 'Karnataka', 12.9716, 77.5946, 12327000, false
FROM countries c WHERE c.iso2 = 'IN'
ON CONFLICT (name, country_id) DO NOTHING;

INSERT INTO cities (id, country_id, name, state_name, latitude, longitude, population, is_capital) 
SELECT gen_random_uuid(), c.id, 'Chennai', 'Tamil Nadu', 13.0827, 80.2707, 10971000, false
FROM countries c WHERE c.iso2 = 'IN'
ON CONFLICT (name, country_id) DO NOTHING;

INSERT INTO cities (id, country_id, name, state_name, latitude, longitude, population, is_capital) 
SELECT gen_random_uuid(), c.id, 'Hyderabad', 'Telangana', 17.3850, 78.4867, 10534000, false
FROM countries c WHERE c.iso2 = 'IN'
ON CONFLICT (name, country_id) DO NOTHING;

INSERT INTO cities (id, country_id, name, state_name, latitude, longitude, population, is_capital) 
SELECT gen_random_uuid(), c.id, 'Pune', 'Maharashtra', 18.5204, 73.8567, 6276000, false
FROM countries c WHERE c.iso2 = 'IN'
ON CONFLICT (name, country_id) DO NOTHING;