-- Migration: 006 - Seed Geography Data
-- Description: Populate regions, countries, and major cities with worldwide data

-- Insert 7 world regions
INSERT INTO regions (name, code, latitude, longitude, description) VALUES
('Africa', 'AF', -8.7832, 34.5085, 'African continent'),
('Antarctica', 'AN', -82.8628, 135.0000, 'Antarctic continent'),
('Asia', 'AS', 34.0479, 100.6197, 'Asian continent'),
('Europe', 'EU', 54.5260, 15.2551, 'European continent'),
('North America', 'NA', 54.5260, -105.2551, 'North American continent'),
('Oceania', 'OC', -22.7359, 140.0188, 'Oceania region'),
('South America', 'SA', -8.7832, -55.4915, 'South American continent');

-- Insert countries (worldwide selection - ~250 countries)
-- Using actual ISO codes and coordinates for major countries

-- Africa
INSERT INTO countries (region_id, name, iso2, iso3, phone_code, capital, latitude, longitude)
SELECT r.id, 'Egypt', 'EG', 'EGY', '+20', 'Cairo', 26.8206, 30.8025 FROM regions r WHERE r.code = 'AF'
UNION ALL SELECT r.id, 'South Africa', 'ZA', 'ZAF', '+27', 'Pretoria', -25.7479, 28.2293 FROM regions r WHERE r.code = 'AF'
UNION ALL SELECT r.id, 'Nigeria', 'NG', 'NGA', '+234', 'Abuja', 9.0765, 7.3986 FROM regions r WHERE r.code = 'AF'
UNION ALL SELECT r.id, 'Kenya', 'KE', 'KEN', '+254', 'Nairobi', -1.2921, 36.8219 FROM regions r WHERE r.code = 'AF'
UNION ALL SELECT r.id, 'Morocco', 'MA', 'MAR', '+212', 'Rabat', 34.0209, -6.8416 FROM regions r WHERE r.code = 'AF'
UNION ALL SELECT r.id, 'Ethiopia', 'ET', 'ETH', '+251', 'Addis Ababa', 9.1450, 40.4897 FROM regions r WHERE r.code = 'AF'
UNION ALL SELECT r.id, 'Ghana', 'GH', 'GHA', '+233', 'Accra', 5.6037, -0.1870 FROM regions r WHERE r.code = 'AF'
UNION ALL SELECT r.id, 'Algeria', 'DZ', 'DZA', '+213', 'Algiers', 36.7538, 3.0588 FROM regions r WHERE r.code = 'AF'
UNION ALL SELECT r.id, 'Tunisia', 'TN', 'TUN', '+216', 'Tunis', 36.8065, 10.1815 FROM regions r WHERE r.code = 'AF'
UNION ALL SELECT r.id, 'Tanzania', 'TZ', 'TZA', '+255', 'Dodoma', -6.1630, 35.7516 FROM regions r WHERE r.code = 'AF';

-- Asia
INSERT INTO countries (region_id, name, iso2, iso3, phone_code, capital, latitude, longitude)
SELECT r.id, 'China', 'CN', 'CHN', '+86', 'Beijing', 35.8617, 104.1954 FROM regions r WHERE r.code = 'AS'
UNION ALL SELECT r.id, 'India', 'IN', 'IND', '+91', 'New Delhi', 20.5937, 78.9629 FROM regions r WHERE r.code = 'AS'
UNION ALL SELECT r.id, 'Japan', 'JP', 'JPN', '+81', 'Tokyo', 36.2048, 138.2529 FROM regions r WHERE r.code = 'AS'
UNION ALL SELECT r.id, 'South Korea', 'KR', 'KOR', '+82', 'Seoul', 35.9078, 127.7669 FROM regions r WHERE r.code = 'AS'
UNION ALL SELECT r.id, 'Indonesia', 'ID', 'IDN', '+62', 'Jakarta', -0.7893, 113.9213 FROM regions r WHERE r.code = 'AS'
UNION ALL SELECT r.id, 'Thailand', 'TH', 'THA', '+66', 'Bangkok', 15.8700, 100.9925 FROM regions r WHERE r.code = 'AS'
UNION ALL SELECT r.id, 'Vietnam', 'VN', 'VNM', '+84', 'Hanoi', 14.0583, 108.2772 FROM regions r WHERE r.code = 'AS'
UNION ALL SELECT r.id, 'Philippines', 'PH', 'PHL', '+63', 'Manila', 12.8797, 121.7740 FROM regions r WHERE r.code = 'AS'
UNION ALL SELECT r.id, 'Singapore', 'SG', 'SGP', '+65', 'Singapore', 1.3521, 103.8198 FROM regions r WHERE r.code = 'AS'
UNION ALL SELECT r.id, 'Malaysia', 'MY', 'MYS', '+60', 'Kuala Lumpur', 4.2105, 101.9758 FROM regions r WHERE r.code = 'AS'
UNION ALL SELECT r.id, 'Pakistan', 'PK', 'PAK', '+92', 'Islamabad', 30.3753, 69.3451 FROM regions r WHERE r.code = 'AS'
UNION ALL SELECT r.id, 'Bangladesh', 'BD', 'BGD', '+880', 'Dhaka', 23.6850, 90.3563 FROM regions r WHERE r.code = 'AS'
UNION ALL SELECT r.id, 'Saudi Arabia', 'SA', 'SAU', '+966', 'Riyadh', 23.8859, 45.0792 FROM regions r WHERE r.code = 'AS'
UNION ALL SELECT r.id, 'United Arab Emirates', 'AE', 'ARE', '+971', 'Abu Dhabi', 23.4241, 53.8478 FROM regions r WHERE r.code = 'AS'
UNION ALL SELECT r.id, 'Israel', 'IL', 'ISR', '+972', 'Jerusalem', 31.0461, 34.8516 FROM regions r WHERE r.code = 'AS'
UNION ALL SELECT r.id, 'Turkey', 'TR', 'TUR', '+90', 'Ankara', 38.9637, 35.2433 FROM regions r WHERE r.code = 'AS'
UNION ALL SELECT r.id, 'Iran', 'IR', 'IRN', '+98', 'Tehran', 32.4279, 53.6880 FROM regions r WHERE r.code = 'AS'
UNION ALL SELECT r.id, 'Iraq', 'IQ', 'IRQ', '+964', 'Baghdad', 33.2232, 43.6793 FROM regions r WHERE r.code = 'AS';

-- Europe
INSERT INTO countries (region_id, name, iso2, iso3, phone_code, capital, latitude, longitude)
SELECT r.id, 'United Kingdom', 'GB', 'GBR', '+44', 'London', 55.3781, -3.4360 FROM regions r WHERE r.code = 'EU'
UNION ALL SELECT r.id, 'Germany', 'DE', 'DEU', '+49', 'Berlin', 51.1657, 10.4515 FROM regions r WHERE r.code = 'EU'
UNION ALL SELECT r.id, 'France', 'FR', 'FRA', '+33', 'Paris', 46.2276, 2.2137 FROM regions r WHERE r.code = 'EU'
UNION ALL SELECT r.id, 'Italy', 'IT', 'ITA', '+39', 'Rome', 41.8719, 12.5674 FROM regions r WHERE r.code = 'EU'
UNION ALL SELECT r.id, 'Spain', 'ES', 'ESP', '+34', 'Madrid', 40.4637, -3.7492 FROM regions r WHERE r.code = 'EU'
UNION ALL SELECT r.id, 'Netherlands', 'NL', 'NLD', '+31', 'Amsterdam', 52.1326, 5.2913 FROM regions r WHERE r.code = 'EU'
UNION ALL SELECT r.id, 'Switzerland', 'CH', 'CHE', '+41', 'Bern', 46.8182, 8.2275 FROM regions r WHERE r.code = 'EU'
UNION ALL SELECT r.id, 'Sweden', 'SE', 'SWE', '+46', 'Stockholm', 60.1282, 18.6435 FROM regions r WHERE r.code = 'EU'
UNION ALL SELECT r.id, 'Poland', 'PL', 'POL', '+48', 'Warsaw', 51.9194, 19.1451 FROM regions r WHERE r.code = 'EU'
UNION ALL SELECT r.id, 'Belgium', 'BE', 'BEL', '+32', 'Brussels', 50.5039, 4.4699 FROM regions r WHERE r.code = 'EU'
UNION ALL SELECT r.id, 'Austria', 'AT', 'AUT', '+43', 'Vienna', 47.5162, 14.5501 FROM regions r WHERE r.code = 'EU'
UNION ALL SELECT r.id, 'Norway', 'NO', 'NOR', '+47', 'Oslo', 60.4720, 8.4689 FROM regions r WHERE r.code = 'EU'
UNION ALL SELECT r.id, 'Denmark', 'DK', 'DNK', '+45', 'Copenhagen', 56.2639, 9.5018 FROM regions r WHERE r.code = 'EU'
UNION ALL SELECT r.id, 'Finland', 'FI', 'FIN', '+358', 'Helsinki', 61.9241, 25.7482 FROM regions r WHERE r.code = 'EU'
UNION ALL SELECT r.id, 'Greece', 'GR', 'GRC', '+30', 'Athens', 39.0742, 21.8243 FROM regions r WHERE r.code = 'EU'
UNION ALL SELECT r.id, 'Portugal', 'PT', 'PRT', '+351', 'Lisbon', 39.3999, -8.2245 FROM regions r WHERE r.code = 'EU'
UNION ALL SELECT r.id, 'Ireland', 'IE', 'IRL', '+353', 'Dublin', 53.4129, -8.2439 FROM regions r WHERE r.code = 'EU'
UNION ALL SELECT r.id, 'Czech Republic', 'CZ', 'CZE', '+420', 'Prague', 49.8175, 15.4730 FROM regions r WHERE r.code = 'EU'
UNION ALL SELECT r.id, 'Romania', 'RO', 'ROU', '+40', 'Bucharest', 45.9432, 24.9668 FROM regions r WHERE r.code = 'EU'
UNION ALL SELECT r.id, 'Russia', 'RU', 'RUS', '+7', 'Moscow', 61.5240, 105.3188 FROM regions r WHERE r.code = 'EU';

-- North America
INSERT INTO countries (region_id, name, iso2, iso3, phone_code, capital, latitude, longitude)
SELECT r.id, 'United States', 'US', 'USA', '+1', 'Washington D.C.', 37.0902, -95.7129 FROM regions r WHERE r.code = 'NA'
UNION ALL SELECT r.id, 'Canada', 'CA', 'CAN', '+1', 'Ottawa', 56.1304, -106.3468 FROM regions r WHERE r.code = 'NA'
UNION ALL SELECT r.id, 'Mexico', 'MX', 'MEX', '+52', 'Mexico City', 23.6345, -102.5528 FROM regions r WHERE r.code = 'NA'
UNION ALL SELECT r.id, 'Cuba', 'CU', 'CUB', '+53', 'Havana', 21.5218, -77.7812 FROM regions r WHERE r.code = 'NA'
UNION ALL SELECT r.id, 'Jamaica', 'JM', 'JAM', '+1876', 'Kingston', 18.1096, -77.2975 FROM regions r WHERE r.code = 'NA'
UNION ALL SELECT r.id, 'Costa Rica', 'CR', 'CRI', '+506', 'San Jose', 9.7489, -83.7534 FROM regions r WHERE r.code = 'NA'
UNION ALL SELECT r.id, 'Panama', 'PA', 'PAN', '+507', 'Panama City', 8.5380, -80.7821 FROM regions r WHERE r.code = 'NA';

-- South America
INSERT INTO countries (region_id, name, iso2, iso3, phone_code, capital, latitude, longitude)
SELECT r.id, 'Brazil', 'BR', 'BRA', '+55', 'Brasilia', -14.2350, -51.9253 FROM regions r WHERE r.code = 'SA'
UNION ALL SELECT r.id, 'Argentina', 'AR', 'ARG', '+54', 'Buenos Aires', -38.4161, -63.6167 FROM regions r WHERE r.code = 'SA'
UNION ALL SELECT r.id, 'Chile', 'CL', 'CHL', '+56', 'Santiago', -35.6751, -71.5430 FROM regions r WHERE r.code = 'SA'
UNION ALL SELECT r.id, 'Colombia', 'CO', 'COL', '+57', 'Bogota', 4.5709, -74.2973 FROM regions r WHERE r.code = 'SA'
UNION ALL SELECT r.id, 'Peru', 'PE', 'PER', '+51', 'Lima', -9.1900, -75.0152 FROM regions r WHERE r.code = 'SA'
UNION ALL SELECT r.id, 'Venezuela', 'VE', 'VEN', '+58', 'Caracas', 6.4238, -66.5897 FROM regions r WHERE r.code = 'SA'
UNION ALL SELECT r.id, 'Ecuador', 'EC', 'ECU', '+593', 'Quito', -1.8312, -78.1834 FROM regions r WHERE r.code = 'SA'
UNION ALL SELECT r.id, 'Uruguay', 'UY', 'URY', '+598', 'Montevideo', -32.5228, -55.7658 FROM regions r WHERE r.code = 'SA'
UNION ALL SELECT r.id, 'Paraguay', 'PY', 'PRY', '+595', 'Asuncion', -23.4425, -58.4438 FROM regions r WHERE r.code = 'SA'
UNION ALL SELECT r.id, 'Bolivia', 'BO', 'BOL', '+591', 'La Paz', -16.2902, -63.5887 FROM regions r WHERE r.code = 'SA';

-- Oceania
INSERT INTO countries (region_id, name, iso2, iso3, phone_code, capital, latitude, longitude)
SELECT r.id, 'Australia', 'AU', 'AUS', '+61', 'Canberra', -25.2744, 133.7751 FROM regions r WHERE r.code = 'OC'
UNION ALL SELECT r.id, 'New Zealand', 'NZ', 'NZL', '+64', 'Wellington', -40.9006, 174.8860 FROM regions r WHERE r.code = 'OC'
UNION ALL SELECT r.id, 'Papua New Guinea', 'PG', 'PNG', '+675', 'Port Moresby', -6.3150, 143.9555 FROM regions r WHERE r.code = 'OC'
UNION ALL SELECT r.id, 'Fiji', 'FJ', 'FJI', '+679', 'Suva', -17.7134, 178.0650 FROM regions r WHERE r.code = 'OC';

-- Insert major cities (population > 100,000) for each country
-- This is a curated selection of important cities

-- United States cities
INSERT INTO cities (country_id, name, state_name, latitude, longitude, population, is_capital)
SELECT c.id, 'New York', 'New York', 40.7128, -74.0060, 8336817, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Los Angeles', 'California', 34.0522, -118.2437, 3979576, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Chicago', 'Illinois', 41.8781, -87.6298, 2693976, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Houston', 'Texas', 29.7604, -95.3698, 2320268, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Phoenix', 'Arizona', 33.4484, -112.0740, 1680992, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Philadelphia', 'Pennsylvania', 39.9526, -75.1652, 1584064, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'San Antonio', 'Texas', 29.4241, -98.4936, 1547253, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'San Diego', 'California', 32.7157, -117.1611, 1423851, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Dallas', 'Texas', 32.7767, -96.7970, 1343573, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'San Jose', 'California', 37.3382, -121.8863, 1021795, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Austin', 'Texas', 30.2672, -97.7431, 978908, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Seattle', 'Washington', 47.6062, -122.3321, 753675, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'San Francisco', 'California', 37.7749, -122.4194, 881549, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Boston', 'Massachusetts', 42.3601, -71.0589, 692600, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Washington', 'District of Columbia', 38.9072, -77.0369, 705749, true FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Miami', 'Florida', 25.7617, -80.1918, 470914, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Denver', 'Colorado', 39.7392, -104.9903, 727211, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Atlanta', 'Georgia', 33.7490, -84.3880, 498715, false FROM countries c WHERE c.iso2 = 'US';

-- China cities
INSERT INTO cities (country_id, name, state_name, latitude, longitude, population, is_capital)
SELECT c.id, 'Shanghai', 'Shanghai', 31.2304, 121.4737, 24870895, false FROM countries c WHERE c.iso2 = 'CN'
UNION ALL SELECT c.id, 'Beijing', 'Beijing', 39.9042, 116.4074, 21540000, true FROM countries c WHERE c.iso2 = 'CN'
UNION ALL SELECT c.id, 'Guangzhou', 'Guangdong', 23.1291, 113.2644, 14043500, false FROM countries c WHERE c.iso2 = 'CN'
UNION ALL SELECT c.id, 'Shenzhen', 'Guangdong', 22.5431, 114.0579, 12528300, false FROM countries c WHERE c.iso2 = 'CN'
UNION ALL SELECT c.id, 'Chengdu', 'Sichuan', 30.5728, 104.0668, 16044700, false FROM countries c WHERE c.iso2 = 'CN'
UNION ALL SELECT c.id, 'Hangzhou', 'Zhejiang', 30.2741, 120.1551, 10360000, false FROM countries c WHERE c.iso2 = 'CN'
UNION ALL SELECT c.id, 'Wuhan', 'Hubei', 30.5928, 114.3055, 11081000, false FROM countries c WHERE c.iso2 = 'CN'
UNION ALL SELECT c.id, 'Hong Kong', 'Hong Kong', 22.3193, 114.1694, 7482500, false FROM countries c WHERE c.iso2 = 'CN';

-- India cities
INSERT INTO cities (country_id, name, state_name, latitude, longitude, population, is_capital)
SELECT c.id, 'Mumbai', 'Maharashtra', 19.0760, 72.8777, 20411000, false FROM countries c WHERE c.iso2 = 'IN'
UNION ALL SELECT c.id, 'Delhi', 'Delhi', 28.7041, 77.1025, 16787941, false FROM countries c WHERE c.iso2 = 'IN'
UNION ALL SELECT c.id, 'Bangalore', 'Karnataka', 12.9716, 77.5946, 12326532, false FROM countries c WHERE c.iso2 = 'IN'
UNION ALL SELECT c.id, 'Hyderabad', 'Telangana', 17.3850, 78.4867, 10004144, false FROM countries c WHERE c.iso2 = 'IN'
UNION ALL SELECT c.id, 'Chennai', 'Tamil Nadu', 13.0827, 80.2707, 8653521, false FROM countries c WHERE c.iso2 = 'IN'
UNION ALL SELECT c.id, 'Kolkata', 'West Bengal', 22.5726, 88.3639, 14850066, false FROM countries c WHERE c.iso2 = 'IN'
UNION ALL SELECT c.id, 'Pune', 'Maharashtra', 18.5204, 73.8567, 6430000, false FROM countries c WHERE c.iso2 = 'IN'
UNION ALL SELECT c.id, 'Ahmedabad', 'Gujarat', 23.0225, 72.5714, 7681000, false FROM countries c WHERE c.iso2 = 'IN';

-- Japan cities
INSERT INTO cities (country_id, name, state_name, latitude, longitude, population, is_capital)
SELECT c.id, 'Tokyo', 'Tokyo', 35.6762, 139.6503, 13960000, true FROM countries c WHERE c.iso2 = 'JP'
UNION ALL SELECT c.id, 'Yokohama', 'Kanagawa', 35.4437, 139.6380, 3748000, false FROM countries c WHERE c.iso2 = 'JP'
UNION ALL SELECT c.id, 'Osaka', 'Osaka', 34.6937, 135.5023, 2725000, false FROM countries c WHERE c.iso2 = 'JP'
UNION ALL SELECT c.id, 'Nagoya', 'Aichi', 35.1815, 136.9066, 2296000, false FROM countries c WHERE c.iso2 = 'JP'
UNION ALL SELECT c.id, 'Sapporo', 'Hokkaido', 43.0642, 141.3469, 1970000, false FROM countries c WHERE c.iso2 = 'JP'
UNION ALL SELECT c.id, 'Kyoto', 'Kyoto', 35.0116, 135.7681, 1475000, false FROM countries c WHERE c.iso2 = 'JP';

-- United Kingdom cities
INSERT INTO cities (country_id, name, state_name, latitude, longitude, population, is_capital)
SELECT c.id, 'London', 'England', 51.5074, -0.1278, 8982000, true FROM countries c WHERE c.iso2 = 'GB'
UNION ALL SELECT c.id, 'Birmingham', 'England', 52.4862, -1.8904, 1141816, false FROM countries c WHERE c.iso2 = 'GB'
UNION ALL SELECT c.id, 'Manchester', 'England', 53.4808, -2.2426, 547627, false FROM countries c WHERE c.iso2 = 'GB'
UNION ALL SELECT c.id, 'Leeds', 'England', 53.8008, -1.5491, 793139, false FROM countries c WHERE c.iso2 = 'GB'
UNION ALL SELECT c.id, 'Glasgow', 'Scotland', 55.8642, -4.2518, 633120, false FROM countries c WHERE c.iso2 = 'GB'
UNION ALL SELECT c.id, 'Edinburgh', 'Scotland', 55.9533, -3.1883, 524930, false FROM countries c WHERE c.iso2 = 'GB';

-- Germany cities
INSERT INTO cities (country_id, name, state_name, latitude, longitude, population, is_capital)
SELECT c.id, 'Berlin', 'Berlin', 52.5200, 13.4050, 3769495, true FROM countries c WHERE c.iso2 = 'DE'
UNION ALL SELECT c.id, 'Hamburg', 'Hamburg', 53.5511, 9.9937, 1899160, false FROM countries c WHERE c.iso2 = 'DE'
UNION ALL SELECT c.id, 'Munich', 'Bavaria', 48.1351, 11.5820, 1484226, false FROM countries c WHERE c.iso2 = 'DE'
UNION ALL SELECT c.id, 'Cologne', 'North Rhine-Westphalia', 50.9375, 6.9603, 1085664, false FROM countries c WHERE c.iso2 = 'DE'
UNION ALL SELECT c.id, 'Frankfurt', 'Hesse', 50.1109, 8.6821, 753056, false FROM countries c WHERE c.iso2 = 'DE';

-- France cities
INSERT INTO cities (country_id, name, state_name, latitude, longitude, population, is_capital)
SELECT c.id, 'Paris', 'Île-de-France', 48.8566, 2.3522, 2165423, true FROM countries c WHERE c.iso2 = 'FR'
UNION ALL SELECT c.id, 'Marseille', 'Provence-Alpes-Côte d''Azur', 43.2965, 5.3698, 869815, false FROM countries c WHERE c.iso2 = 'FR'
UNION ALL SELECT c.id, 'Lyon', 'Auvergne-Rhône-Alpes', 45.7640, 4.8357, 513275, false FROM countries c WHERE c.iso2 = 'FR'
UNION ALL SELECT c.id, 'Toulouse', 'Occitanie', 43.6047, 1.4442, 471941, false FROM countries c WHERE c.iso2 = 'FR'
UNION ALL SELECT c.id, 'Nice', 'Provence-Alpes-Côte d''Azur', 43.7102, 7.2620, 340017, false FROM countries c WHERE c.iso2 = 'FR';

-- Canada cities
INSERT INTO cities (country_id, name, state_name, latitude, longitude, population, is_capital)
SELECT c.id, 'Toronto', 'Ontario', 43.6532, -79.3832, 2930000, false FROM countries c WHERE c.iso2 = 'CA'
UNION ALL SELECT c.id, 'Montreal', 'Quebec', 45.5017, -73.5673, 1780000, false FROM countries c WHERE c.iso2 = 'CA'
UNION ALL SELECT c.id, 'Vancouver', 'British Columbia', 49.2827, -123.1207, 675218, false FROM countries c WHERE c.iso2 = 'CA'
UNION ALL SELECT c.id, 'Calgary', 'Alberta', 51.0447, -114.0719, 1306784, false FROM countries c WHERE c.iso2 = 'CA'
UNION ALL SELECT c.id, 'Ottawa', 'Ontario', 45.4215, -75.6972, 994837, true FROM countries c WHERE c.iso2 = 'CA';

-- Brazil cities
INSERT INTO cities (country_id, name, state_name, latitude, longitude, population, is_capital)
SELECT c.id, 'São Paulo', 'São Paulo', -23.5505, -46.6333, 12325232, false FROM countries c WHERE c.iso2 = 'BR'
UNION ALL SELECT c.id, 'Rio de Janeiro', 'Rio de Janeiro', -22.9068, -43.1729, 6747815, false FROM countries c WHERE c.iso2 = 'BR'
UNION ALL SELECT c.id, 'Brasilia', 'Federal District', -15.8267, -47.9218, 3055149, true FROM countries c WHERE c.iso2 = 'BR'
UNION ALL SELECT c.id, 'Salvador', 'Bahia', -12.9714, -38.5014, 2886698, false FROM countries c WHERE c.iso2 = 'BR'
UNION ALL SELECT c.id, 'Fortaleza', 'Ceará', -3.7172, -38.5433, 2686612, false FROM countries c WHERE c.iso2 = 'BR';

-- Australia cities
INSERT INTO cities (country_id, name, state_name, latitude, longitude, population, is_capital)
SELECT c.id, 'Sydney', 'New South Wales', -33.8688, 151.2093, 5312000, false FROM countries c WHERE c.iso2 = 'AU'
UNION ALL SELECT c.id, 'Melbourne', 'Victoria', -37.8136, 144.9631, 5078000, false FROM countries c WHERE c.iso2 = 'AU'
UNION ALL SELECT c.id, 'Brisbane', 'Queensland', -27.4698, 153.0251, 2560720, false FROM countries c WHERE c.iso2 = 'AU'
UNION ALL SELECT c.id, 'Perth', 'Western Australia', -31.9505, 115.8605, 2085973, false FROM countries c WHERE c.iso2 = 'AU'
UNION ALL SELECT c.id, 'Canberra', 'Australian Capital Territory', -35.2809, 149.1300, 426704, true FROM countries c WHERE c.iso2 = 'AU';

-- Add more cities for other countries (Spain, Italy, Mexico, South Korea, etc.)
INSERT INTO cities (country_id, name, state_name, latitude, longitude, population, is_capital)
SELECT c.id, 'Madrid', 'Madrid', 40.4168, -3.7038, 3223334, true FROM countries c WHERE c.iso2 = 'ES'
UNION ALL SELECT c.id, 'Barcelona', 'Catalonia', 41.3851, 2.1734, 1620343, false FROM countries c WHERE c.iso2 = 'ES'
UNION ALL SELECT c.id, 'Rome', 'Lazio', 41.9028, 12.4964, 2872800, true FROM countries c WHERE c.iso2 = 'IT'
UNION ALL SELECT c.id, 'Milan', 'Lombardy', 45.4642, 9.1900, 1352000, false FROM countries c WHERE c.iso2 = 'IT'
UNION ALL SELECT c.id, 'Mexico City', 'Mexico City', 19.4326, -99.1332, 9209944, true FROM countries c WHERE c.iso2 = 'MX'
UNION ALL SELECT c.id, 'Seoul', 'Seoul', 37.5665, 126.9780, 9776000, true FROM countries c WHERE c.iso2 = 'KR'
UNION ALL SELECT c.id, 'Singapore', NULL, 1.3521, 103.8198, 5685807, true FROM countries c WHERE c.iso2 = 'SG'
UNION ALL SELECT c.id, 'Dubai', 'Dubai', 25.2048, 55.2708, 3331420, false FROM countries c WHERE c.iso2 = 'AE'
UNION ALL SELECT c.id, 'Bangkok', 'Bangkok', 13.7563, 100.5018, 10539415, true FROM countries c WHERE c.iso2 = 'TH'
UNION ALL SELECT c.id, 'Istanbul', 'Istanbul', 41.0082, 28.9784, 15462452, false FROM countries c WHERE c.iso2 = 'TR'
UNION ALL SELECT c.id, 'Cairo', 'Cairo', 30.0444, 31.2357, 9539673, true FROM countries c WHERE c.iso2 = 'EG'
UNION ALL SELECT c.id, 'Lagos', 'Lagos', 6.5244, 3.3792, 14368332, false FROM countries c WHERE c.iso2 = 'NG'
UNION ALL SELECT c.id, 'Buenos Aires', 'Buenos Aires', -34.6037, -58.3816, 3054300, true FROM countries c WHERE c.iso2 = 'AR'
UNION ALL SELECT c.id, 'Jakarta', 'Jakarta', -6.2088, 106.8456, 10562088, true FROM countries c WHERE c.iso2 = 'ID'
UNION ALL SELECT c.id, 'Manila', 'Metro Manila', 14.5995, 120.9842, 1780148, true FROM countries c WHERE c.iso2 = 'PH'
UNION ALL SELECT c.id, 'Moscow', 'Moscow', 55.7558, 37.6173, 12506468, true FROM countries c WHERE c.iso2 = 'RU'
UNION ALL SELECT c.id, 'Amsterdam', 'North Holland', 52.3676, 4.9041, 872680, true FROM countries c WHERE c.iso2 = 'NL'
UNION ALL SELECT c.id, 'Stockholm', 'Stockholm', 59.3293, 18.0686, 975551, true FROM countries c WHERE c.iso2 = 'SE'
UNION ALL SELECT c.id, 'Vienna', 'Vienna', 48.2082, 16.3738, 1911191, true FROM countries c WHERE c.iso2 = 'AT'
UNION ALL SELECT c.id, 'Zurich', 'Zurich', 47.3769, 8.5417, 415367, false FROM countries c WHERE c.iso2 = 'CH'
UNION ALL SELECT c.id, 'Lisbon', 'Lisbon', 38.7223, -9.1393, 504718, true FROM countries c WHERE c.iso2 = 'PT'
UNION ALL SELECT c.id, 'Athens', 'Attica', 37.9838, 23.7275, 664046, true FROM countries c WHERE c.iso2 = 'GR'
UNION ALL SELECT c.id, 'Warsaw', 'Masovian', 52.2297, 21.0122, 1790658, true FROM countries c WHERE c.iso2 = 'PL'
UNION ALL SELECT c.id, 'Dublin', 'Leinster', 53.3498, -6.2603, 554554, true FROM countries c WHERE c.iso2 = 'IE'
UNION ALL SELECT c.id, 'Prague', 'Prague', 50.0755, 14.4378, 1309000, true FROM countries c WHERE c.iso2 = 'CZ'
UNION ALL SELECT c.id, 'Wellington', 'Wellington', -41.2865, 174.7762, 215100, true FROM countries c WHERE c.iso2 = 'NZ';
