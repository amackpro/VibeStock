-- Migration 007: Comprehensive Geographic Data & Sample Suppliers
-- This adds more cities (20+ per country) and creates sample suppliers for testing

-- Note: This assumes migration 006 has already been run (regions and initial countries exist)

-- ============================================================================
-- PART 1: ADD MORE CITIES TO EXISTING COUNTRIES
-- ============================================================================

-- United States - 50 major cities
INSERT INTO cities (country_id, name, state_name, latitude, longitude, population, is_capital)
SELECT c.id, 'Las Vegas', 'Nevada', 36.1699, -115.1398, 641676, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Portland', 'Oregon', 45.5152, -122.6784, 652503, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Nashville', 'Tennessee', 36.1627, -86.7816, 689447, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Detroit', 'Michigan', 42.3314, -83.0458, 670031, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Memphis', 'Tennessee', 35.1495, -90.0490, 651073, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Louisville', 'Kentucky', 38.2527, -85.7585, 617638, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Baltimore', 'Maryland', 39.2904, -76.6122, 585708, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Milwaukee', 'Wisconsin', 43.0389, -87.9065, 590157, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Charlotte', 'North Carolina', 35.2271, -80.8431, 885708, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Columbus', 'Ohio', 39.9612, -82.9988, 898553, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Indianapolis', 'Indiana', 39.7684, -86.1581, 876384, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Jacksonville', 'Florida', 30.3322, -81.6557, 911507, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Fort Worth', 'Texas', 32.7555, -97.3308, 918915, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'El Paso', 'Texas', 31.7619, -106.4850, 681728, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Oklahoma City', 'Oklahoma', 35.4676, -97.5164, 687725, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Tucson', 'Arizona', 32.2226, -110.9747, 548073, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Minneapolis', 'Minnesota', 44.9778, -93.2650, 429606, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'New Orleans', 'Louisiana', 29.9511, -90.0715, 383997, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Cleveland', 'Ohio', 41.4993, -81.6944, 372624, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Tampa', 'Florida', 27.9506, -82.4572, 399700, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Raleigh', 'North Carolina', 35.7796, -78.6382, 474069, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'St. Louis', 'Missouri', 38.6270, -90.1994, 301578, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Pittsburgh', 'Pennsylvania', 40.4406, -79.9959, 302971, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Cincinnati', 'Ohio', 39.1031, -84.5120, 309317, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Orlando', 'Florida', 28.5383, -81.3792, 307573, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Sacramento', 'California', 38.5816, -121.4944, 524943, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Kansas City', 'Missouri', 39.0997, -94.5786, 508090, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Omaha', 'Nebraska', 41.2565, -95.9345, 486051, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Albuquerque', 'New Mexico', 35.0844, -106.6504, 564559, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Fresno', 'California', 36.7378, -119.7871, 542107, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Mesa', 'Arizona', 33.4152, -111.8315, 518012, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Virginia Beach', 'Virginia', 36.8529, -75.9780, 459470, false FROM countries c WHERE c.iso2 = 'US'
UNION ALL SELECT c.id, 'Long Beach', 'California', 33.7701, -118.1937, 466742, false FROM countries c WHERE c.iso2 = 'US'
ON CONFLICT (id) DO NOTHING;

-- China - 30 major cities
INSERT INTO cities (country_id, name, state_name, latitude, longitude, population, is_capital)
SELECT c.id, 'Tianjin', 'Tianjin', 39.3434, 117.3616, 15621000, false FROM countries c WHERE c.iso2 = 'CN'
UNION ALL SELECT c.id, 'Chongqing', 'Chongqing', 29.4316, 106.9123, 15872179, false FROM countries c WHERE c.iso2 = 'CN'
UNION ALL SELECT c.id, 'Dongguan', 'Guangdong', 23.0209, 113.7518, 8220237, false FROM countries c WHERE c.iso2 = 'CN'
UNION ALL SELECT c.id, 'Nanjing', 'Jiangsu', 32.0603, 118.7969, 8505000, false FROM countries c WHERE c.iso2 = 'CN'
UNION ALL SELECT c.id, 'Shenyang', 'Liaoning', 41.8057, 123.4328, 8294000, false FROM countries c WHERE c.iso2 = 'CN'
UNION ALL SELECT c.id, 'Harbin', 'Heilongjiang', 45.8038, 126.5350, 10635971, false FROM countries c WHERE c.iso2 = 'CN'
UNION ALL SELECT c.id, 'Xi''an', 'Shaanxi', 34.3416, 108.9398, 12005600, false FROM countries c WHERE c.iso2 = 'CN'
UNION ALL SELECT c.id, 'Suzhou', 'Jiangsu', 31.2989, 120.5853, 10721700, false FROM countries c WHERE c.iso2 = 'CN'
UNION ALL SELECT c.id, 'Qingdao', 'Shandong', 36.0671, 120.3826, 9046200, false FROM countries c WHERE c.iso2 = 'CN'
UNION ALL SELECT c.id, 'Dalian', 'Liaoning', 38.9140, 121.6147, 6690432, false FROM countries c WHERE c.iso2 = 'CN'
UNION ALL SELECT c.id, 'Zhengzhou', 'Henan', 34.7466, 113.6253, 10136000, false FROM countries c WHERE c.iso2 = 'CN'
UNION ALL SELECT c.id, 'Jinan', 'Shandong', 36.6512, 117.1201, 7321200, false FROM countries c WHERE c.iso2 = 'CN'
UNION ALL SELECT c.id, 'Changsha', 'Hunan', 28.2282, 112.9388, 8154800, false FROM countries c WHERE c.iso2 = 'CN'
UNION ALL SELECT c.id, 'Kunming', 'Yunnan', 25.0406, 102.7129, 6850000, false FROM countries c WHERE c.iso2 = 'CN'
UNION ALL SELECT c.id, 'Changchun', 'Jilin', 43.8171, 125.3235, 7674439, false FROM countries c WHERE c.iso2 = 'CN'
UNION ALL SELECT c.id, 'Taiyuan', 'Shanxi', 37.8706, 112.5489, 4201591, false FROM countries c WHERE c.iso2 = 'CN'
UNION ALL SELECT c.id, 'Nanchang', 'Jiangxi', 28.6829, 115.8579, 5042565, false FROM countries c WHERE c.iso2 = 'CN'
UNION ALL SELECT c.id, 'Fuzhou', 'Fujian', 26.0745, 119.2965, 7115370, false FROM countries c WHERE c.iso2 = 'CN'
UNION ALL SELECT c.id, 'Hefei', 'Anhui', 31.8206, 117.2272, 7965300, false FROM countries c WHERE c.iso2 = 'CN'
UNION ALL SELECT c.id, 'Shijiazhuang', 'Hebei', 38.0428, 114.5149, 10784600, false FROM countries c WHERE c.iso2 = 'CN'
UNION ALL SELECT c.id, 'Urumqi', 'Xinjiang', 43.8256, 87.6168, 3500000, false FROM countries c WHERE c.iso2 = 'CN'
UNION ALL SELECT c.id, 'Ningbo', 'Zhejiang', 29.8683, 121.5440, 7605700, false FROM countries c WHERE c.iso2 = 'CN'
ON CONFLICT (id) DO NOTHING;

-- India - 30 major cities
INSERT INTO cities (country_id, name, state_name, latitude, longitude, population, is_capital)
SELECT c.id, 'Surat', 'Gujarat', 21.1702, 72.8311, 6081000, false FROM countries c WHERE c.iso2 = 'IN'
UNION ALL SELECT c.id, 'Jaipur', 'Rajasthan', 26.9124, 75.7873, 3046163, false FROM countries c WHERE c.iso2 = 'IN'
UNION ALL SELECT c.id, 'Lucknow', 'Uttar Pradesh', 26.8467, 80.9462, 2817105, false FROM countries c WHERE c.iso2 = 'IN'
UNION ALL SELECT c.id, 'Kanpur', 'Uttar Pradesh', 26.4499, 80.3319, 2765348, false FROM countries c WHERE c.iso2 = 'IN'
UNION ALL SELECT c.id, 'Nagpur', 'Maharashtra', 21.1458, 79.0882, 2405665, false FROM countries c WHERE c.iso2 = 'IN'
UNION ALL SELECT c.id, 'Indore', 'Madhya Pradesh', 22.7196, 75.8577, 1994397, false FROM countries c WHERE c.iso2 = 'IN'
UNION ALL SELECT c.id, 'Thane', 'Maharashtra', 19.2183, 72.9781, 1841488, false FROM countries c WHERE c.iso2 = 'IN'
UNION ALL SELECT c.id, 'Bhopal', 'Madhya Pradesh', 23.2599, 77.4126, 1798218, false FROM countries c WHERE c.iso2 = 'IN'
UNION ALL SELECT c.id, 'Visakhapatnam', 'Andhra Pradesh', 17.6868, 83.2185, 1730320, false FROM countries c WHERE c.iso2 = 'IN'
UNION ALL SELECT c.id, 'Pimpri-Chinchwad', 'Maharashtra', 18.6298, 73.7997, 1727692, false FROM countries c WHERE c.iso2 = 'IN'
UNION ALL SELECT c.id, 'Patna', 'Bihar', 25.5941, 85.1376, 1684222, false FROM countries c WHERE c.iso2 = 'IN'
UNION ALL SELECT c.id, 'Vadodara', 'Gujarat', 22.3072, 73.1812, 1670806, false FROM countries c WHERE c.iso2 = 'IN'
UNION ALL SELECT c.id, 'Ghaziabad', 'Uttar Pradesh', 28.6692, 77.4538, 1648643, false FROM countries c WHERE c.iso2 = 'IN'
UNION ALL SELECT c.id, 'Ludhiana', 'Punjab', 30.9010, 75.8573, 1618879, false FROM countries c WHERE c.iso2 = 'IN'
UNION ALL SELECT c.id, 'Agra', 'Uttar Pradesh', 27.1767, 78.0081, 1585705, false FROM countries c WHERE c.iso2 = 'IN'
UNION ALL SELECT c.id, 'Nashik', 'Maharashtra', 19.9975, 73.7898, 1486053, false FROM countries c WHERE c.iso2 = 'IN'
UNION ALL SELECT c.id, 'Faridabad', 'Haryana', 28.4089, 77.3178, 1414050, false FROM countries c WHERE c.iso2 = 'IN'
UNION ALL SELECT c.id, 'Meerut', 'Uttar Pradesh', 28.9845, 77.7064, 1305429, false FROM countries c WHERE c.iso2 = 'IN'
UNION ALL SELECT c.id, 'Rajkot', 'Gujarat', 22.3039, 70.8022, 1390640, false FROM countries c WHERE c.iso2 = 'IN'
UNION ALL SELECT c.id, 'Kalyan-Dombivali', 'Maharashtra', 19.2403, 73.1305, 1247327, false FROM countries c WHERE c.iso2 = 'IN'
UNION ALL SELECT c.id, 'Vasai-Virar', 'Maharashtra', 19.4612, 72.7985, 1222390, false FROM countries c WHERE c.iso2 = 'IN'
UNION ALL SELECT c.id, 'Varanasi', 'Uttar Pradesh', 25.3176, 82.9739, 1198491, false FROM countries c WHERE c.iso2 = 'IN'
ON CONFLICT (id) DO NOTHING;

-- United Kingdom - 25 cities
INSERT INTO cities (country_id, name, state_name, latitude, longitude, population, is_capital)
SELECT c.id, 'Liverpool', 'England', 53.4084, -2.9916, 494814, false FROM countries c WHERE c.iso2 = 'GB'
UNION ALL SELECT c.id, 'Newcastle', 'England', 54.9783, -1.6178, 302820, false FROM countries c WHERE c.iso2 = 'GB'
UNION ALL SELECT c.id, 'Sheffield', 'England', 53.3811, -1.4701, 584853, false FROM countries c WHERE c.iso2 = 'GB'
UNION ALL SELECT c.id, 'Bristol', 'England', 51.4545, -2.5879, 463377, false FROM countries c WHERE c.iso2 = 'GB'
UNION ALL SELECT c.id, 'Leicester', 'England', 52.6369, -1.1398, 355218, false FROM countries c WHERE c.iso2 = 'GB'
UNION ALL SELECT c.id, 'Nottingham', 'England', 52.9548, -1.1581, 321500, false FROM countries c WHERE c.iso2 = 'GB'
UNION ALL SELECT c.id, 'Southampton', 'England', 50.9097, -1.4044, 253651, false FROM countries c WHERE c.iso2 = 'GB'
UNION ALL SELECT c.id, 'Brighton', 'England', 50.8225, -0.1372, 290395, false FROM countries c WHERE c.iso2 = 'GB'
UNION ALL SELECT c.id, 'Aberdeen', 'Scotland', 57.1497, -2.0943, 198590, false FROM countries c WHERE c.iso2 = 'GB'
UNION ALL SELECT c.id, 'Cardiff', 'Wales', 51.4816, -3.1791, 364248, false FROM countries c WHERE c.iso2 = 'GB'
UNION ALL SELECT c.id, 'Belfast', 'Northern Ireland', 54.5973, -5.9301, 345418, false FROM countries c WHERE c.iso2 = 'GB'
UNION ALL SELECT c.id, 'Plymouth', 'England', 50.3755, -4.1427, 262100, false FROM countries c WHERE c.iso2 = 'GB'
UNION ALL SELECT c.id, 'Cambridge', 'England', 52.2053, 0.1218, 158434, false FROM countries c WHERE c.iso2 = 'GB'
UNION ALL SELECT c.id, 'Oxford', 'England', 51.7520, -1.2577, 154600, false FROM countries c WHERE c.iso2 = 'GB'
UNION ALL SELECT c.id, 'York', 'England', 53.9591, -1.0815, 209893, false FROM countries c WHERE c.iso2 = 'GB'
UNION ALL SELECT c.id, 'Coventry', 'England', 52.4068, -1.5197, 345385, false FROM countries c WHERE c.iso2 = 'GB'
UNION ALL SELECT c.id, 'Reading', 'England', 51.4543, -0.9781, 174224, false FROM countries c WHERE c.iso2 = 'GB'
ON CONFLICT (id) DO NOTHING;

-- Germany - 25 cities
INSERT INTO cities (country_id, name, state_name, latitude, longitude, population, is_capital)
SELECT c.id, 'Stuttgart', 'Baden-Württemberg', 48.7758, 9.1829, 634830, false FROM countries c WHERE c.iso2 = 'DE'
UNION ALL SELECT c.id, 'Düsseldorf', 'North Rhine-Westphalia', 51.2277, 6.7735, 621877, false FROM countries c WHERE c.iso2 = 'DE'
UNION ALL SELECT c.id, 'Dortmund', 'North Rhine-Westphalia', 51.5136, 7.4653, 587010, false FROM countries c WHERE c.iso2 = 'DE'
UNION ALL SELECT c.id, 'Essen', 'North Rhine-Westphalia', 51.4556, 7.0116, 582760, false FROM countries c WHERE c.iso2 = 'DE'
UNION ALL SELECT c.id, 'Leipzig', 'Saxony', 51.3397, 12.3731, 597493, false FROM countries c WHERE c.iso2 = 'DE'
UNION ALL SELECT c.id, 'Bremen', 'Bremen', 53.0793, 8.8017, 569352, false FROM countries c WHERE c.iso2 = 'DE'
UNION ALL SELECT c.id, 'Dresden', 'Saxony', 51.0504, 13.7373, 556780, false FROM countries c WHERE c.iso2 = 'DE'
UNION ALL SELECT c.id, 'Hanover', 'Lower Saxony', 52.3759, 9.7320, 538068, false FROM countries c WHERE c.iso2 = 'DE'
UNION ALL SELECT c.id, 'Nuremberg', 'Bavaria', 49.4521, 11.0767, 518370, false FROM countries c WHERE c.iso2 = 'DE'
UNION ALL SELECT c.id, 'Duisburg', 'North Rhine-Westphalia', 51.4344, 6.7623, 498686, false FROM countries c WHERE c.iso2 = 'DE'
UNION ALL SELECT c.id, 'Bochum', 'North Rhine-Westphalia', 51.4818, 7.2162, 364920, false FROM countries c WHERE c.iso2 = 'DE'
UNION ALL SELECT c.id, 'Wuppertal', 'North Rhine-Westphalia', 51.2562, 7.1508, 355100, false FROM countries c WHERE c.iso2 = 'DE'
UNION ALL SELECT c.id, 'Bonn', 'North Rhine-Westphalia', 50.7374, 7.0982, 329673, false FROM countries c WHERE c.iso2 = 'DE'
UNION ALL SELECT c.id, 'Mannheim', 'Baden-Württemberg', 49.4875, 8.4660, 309370, false FROM countries c WHERE c.iso2 = 'DE'
UNION ALL SELECT c.id, 'Karlsruhe', 'Baden-Württemberg', 49.0069, 8.4037, 313092, false FROM countries c WHERE c.iso2 = 'DE'
UNION ALL SELECT c.id, 'Münster', 'North Rhine-Westphalia', 51.9607, 7.6261, 315293, false FROM countries c WHERE c.iso2 = 'DE'
UNION ALL SELECT c.id, 'Augsburg', 'Bavaria', 48.3705, 10.8978, 295830, false FROM countries c WHERE c.iso2 = 'DE'
ON CONFLICT (id) DO NOTHING;

-- France - 25 cities
INSERT INTO cities (country_id, name, state_name, latitude, longitude, population, is_capital)
SELECT c.id, 'Nantes', 'Pays de la Loire', 47.2184, -1.5536, 309346, false FROM countries c WHERE c.iso2 = 'FR'
UNION ALL SELECT c.id, 'Strasbourg', 'Grand Est', 48.5734, 7.7521, 280966, false FROM countries c WHERE c.iso2 = 'FR'
UNION ALL SELECT c.id, 'Montpellier', 'Occitanie', 43.6108, 3.8767, 290053, false FROM countries c WHERE c.iso2 = 'FR'
UNION ALL SELECT c.id, 'Bordeaux', 'Nouvelle-Aquitaine', 44.8378, -0.5792, 254436, false FROM countries c WHERE c.iso2 = 'FR'
UNION ALL SELECT c.id, 'Lille', 'Hauts-de-France', 50.6292, 3.0573, 232787, false FROM countries c WHERE c.iso2 = 'FR'
UNION ALL SELECT c.id, 'Rennes', 'Bretagne', 48.1173, -1.6778, 216815, false FROM countries c WHERE c.iso2 = 'FR'
UNION ALL SELECT c.id, 'Reims', 'Grand Est', 49.2583, 4.0317, 183042, false FROM countries c WHERE c.iso2 = 'FR'
UNION ALL SELECT c.id, 'Le Havre', 'Normandie', 49.4944, 0.1079, 170147, false FROM countries c WHERE c.iso2 = 'FR'
UNION ALL SELECT c.id, 'Saint-Étienne', 'Auvergne-Rhône-Alpes', 45.4397, 4.3872, 171057, false FROM countries c WHERE c.iso2 = 'FR'
UNION ALL SELECT c.id, 'Toulon', 'Provence-Alpes-Côte d''Azur', 43.1242, 5.9280, 171953, false FROM countries c WHERE c.iso2 = 'FR'
UNION ALL SELECT c.id, 'Grenoble', 'Auvergne-Rhône-Alpes', 45.1885, 5.7245, 158454, false FROM countries c WHERE c.iso2 = 'FR'
UNION ALL SELECT c.id, 'Dijon', 'Bourgogne-Franche-Comté', 47.3220, 5.0415, 156920, false FROM countries c WHERE c.iso2 = 'FR'
UNION ALL SELECT c.id, 'Angers', 'Pays de la Loire', 47.4784, -0.5632, 152960, false FROM countries c WHERE c.iso2 = 'FR'
UNION ALL SELECT c.id, 'Nîmes', 'Occitanie', 43.8367, 4.3601, 151001, false FROM countries c WHERE c.iso2 = 'FR'
UNION ALL SELECT c.id, 'Villeurbanne', 'Auvergne-Rhône-Alpes', 45.7660, 4.8795, 149019, false FROM countries c WHERE c.iso2 = 'FR'
UNION ALL SELECT c.id, 'Le Mans', 'Pays de la Loire', 48.0077, 0.1984, 143599, false FROM countries c WHERE c.iso2 = 'FR'
UNION ALL SELECT c.id, 'Aix-en-Provence', 'Provence-Alpes-Côte d''Azur', 43.5297, 5.4474, 145133, false FROM countries c WHERE c.iso2 = 'FR'
ON CONFLICT (id) DO NOTHING;

-- Japan - 25 cities
INSERT INTO cities (country_id, name, state_name, latitude, longitude, population, is_capital)
SELECT c.id, 'Kobe', 'Hyogo', 34.6901, 135.1955, 1544000, false FROM countries c WHERE c.iso2 = 'JP'
UNION ALL SELECT c.id, 'Fukuoka', 'Fukuoka', 33.5904, 130.4017, 1581000, false FROM countries c WHERE c.iso2 = 'JP'
UNION ALL SELECT c.id, 'Kawasaki', 'Kanagawa', 35.5307, 139.7029, 1531000, false FROM countries c WHERE c.iso2 = 'JP'
UNION ALL SELECT c.id, 'Saitama', 'Saitama', 35.8617, 139.6455, 1263979, false FROM countries c WHERE c.iso2 = 'JP'
UNION ALL SELECT c.id, 'Hiroshima', 'Hiroshima', 34.3853, 132.4553, 1199000, false FROM countries c WHERE c.iso2 = 'JP'
UNION ALL SELECT c.id, 'Sendai', 'Miyagi', 38.2682, 140.8694, 1096000, false FROM countries c WHERE c.iso2 = 'JP'
UNION ALL SELECT c.id, 'Kitakyushu', 'Fukuoka', 33.8834, 130.8751, 940141, false FROM countries c WHERE c.iso2 = 'JP'
UNION ALL SELECT c.id, 'Chiba', 'Chiba', 35.6074, 140.1065, 980000, false FROM countries c WHERE c.iso2 = 'JP'
UNION ALL SELECT c.id, 'Sakai', 'Osaka', 34.5733, 135.4830, 828741, false FROM countries c WHERE c.iso2 = 'JP'
UNION ALL SELECT c.id, 'Niigata', 'Niigata', 37.9161, 139.0364, 810000, false FROM countries c WHERE c.iso2 = 'JP'
UNION ALL SELECT c.id, 'Hamamatsu', 'Shizuoka', 34.7108, 137.7261, 790000, false FROM countries c WHERE c.iso2 = 'JP'
UNION ALL SELECT c.id, 'Kumamoto', 'Kumamoto', 32.8031, 130.7079, 740000, false FROM countries c WHERE c.iso2 = 'JP'
UNION ALL SELECT c.id, 'Okayama', 'Okayama', 34.6551, 133.9195, 720841, false FROM countries c WHERE c.iso2 = 'JP'
UNION ALL SELECT c.id, 'Shizuoka', 'Shizuoka', 34.9756, 138.3828, 690881, false FROM countries c WHERE c.iso2 = 'JP'
UNION ALL SELECT c.id, 'Sagamihara', 'Kanagawa', 35.5731, 139.3753, 723470, false FROM countries c WHERE c.iso2 = 'JP'
UNION ALL SELECT c.id, 'Kagoshima', 'Kagoshima', 31.5966, 130.5571, 595049, false FROM countries c WHERE c.iso2 = 'JP'
UNION ALL SELECT c.id, 'Nagasaki', 'Nagasaki', 32.7503, 129.8779, 410204, false FROM countries c WHERE c.iso2 = 'JP'
ON CONFLICT (id) DO NOTHING;

-- Canada - 20 cities
INSERT INTO cities (country_id, name, state_name, latitude, longitude, population, is_capital)
SELECT c.id, 'Edmonton', 'Alberta', 53.5461, -113.4938, 972223, false FROM countries c WHERE c.iso2 = 'CA'
UNION ALL SELECT c.id, 'Winnipeg', 'Manitoba', 49.8954, -97.1385, 749534, false FROM countries c WHERE c.iso2 = 'CA'
UNION ALL SELECT c.id, 'Quebec City', 'Quebec', 46.8139, -71.2080, 542298, false FROM countries c WHERE c.iso2 = 'CA'
UNION ALL SELECT c.id, 'Hamilton', 'Ontario', 43.2557, -79.8711, 569353, false FROM countries c WHERE c.iso2 = 'CA'
UNION ALL SELECT c.id, 'Kitchener', 'Ontario', 43.4516, -80.4925, 256885, false FROM countries c WHERE c.iso2 = 'CA'
UNION ALL SELECT c.id, 'London', 'Ontario', 42.9849, -81.2453, 422324, false FROM countries c WHERE c.iso2 = 'CA'
UNION ALL SELECT c.id, 'Victoria', 'British Columbia', 48.4284, -123.3656, 91867, false FROM countries c WHERE c.iso2 = 'CA'
UNION ALL SELECT c.id, 'Halifax', 'Nova Scotia', 44.6488, -63.5752, 439819, false FROM countries c WHERE c.iso2 = 'CA'
UNION ALL SELECT c.id, 'Oshawa', 'Ontario', 43.8971, -78.8658, 166000, false FROM countries c WHERE c.iso2 = 'CA'
UNION ALL SELECT c.id, 'Windsor', 'Ontario', 42.3149, -83.0364, 229660, false FROM countries c WHERE c.iso2 = 'CA'
UNION ALL SELECT c.id, 'Saskatoon', 'Saskatchewan', 52.1332, -106.6700, 273010, false FROM countries c WHERE c.iso2 = 'CA'
UNION ALL SELECT c.id, 'Regina', 'Saskatchewan', 50.4452, -104.6189, 228800, false FROM countries c WHERE c.iso2 = 'CA'
UNION ALL SELECT c.id, 'St. John''s', 'Newfoundland and Labrador', 47.5615, -52.7126, 110525, false FROM countries c WHERE c.iso2 = 'CA'
UNION ALL SELECT c.id, 'Kelowna', 'British Columbia', 49.8880, -119.4960, 142146, false FROM countries c WHERE c.iso2 = 'CA'
UNION ALL SELECT c.id, 'Barrie', 'Ontario', 44.3894, -79.6903, 153000, false FROM countries c WHERE c.iso2 = 'CA'
ON CONFLICT (id) DO NOTHING;

-- Brazil - 20 cities
INSERT INTO cities (country_id, name, state_name, latitude, longitude, population, is_capital)
SELECT c.id, 'Belo Horizonte', 'Minas Gerais', -19.9167, -43.9345, 2521564, false FROM countries c WHERE c.iso2 = 'BR'
UNION ALL SELECT c.id, 'Manaus', 'Amazonas', -3.1190, -60.0217, 2182763, false FROM countries c WHERE c.iso2 = 'BR'
UNION ALL SELECT c.id, 'Curitiba', 'Paraná', -25.4284, -49.2733, 1948626, false FROM countries c WHERE c.iso2 = 'BR'
UNION ALL SELECT c.id, 'Recife', 'Pernambuco', -8.0476, -34.8770, 1653461, false FROM countries c WHERE c.iso2 = 'BR'
UNION ALL SELECT c.id, 'Porto Alegre', 'Rio Grande do Sul', -30.0346, -51.2177, 1488252, false FROM countries c WHERE c.iso2 = 'BR'
UNION ALL SELECT c.id, 'Belém', 'Pará', -1.4558, -48.5039, 1499641, false FROM countries c WHERE c.iso2 = 'BR'
UNION ALL SELECT c.id, 'Goiânia', 'Goiás', -16.6869, -49.2648, 1536097, false FROM countries c WHERE c.iso2 = 'BR'
UNION ALL SELECT c.id, 'Guarulhos', 'São Paulo', -23.4538, -46.5333, 1392121, false FROM countries c WHERE c.iso2 = 'BR'
UNION ALL SELECT c.id, 'Campinas', 'São Paulo', -22.9099, -47.0626, 1213792, false FROM countries c WHERE c.iso2 = 'BR'
UNION ALL SELECT c.id, 'São Luís', 'Maranhão', -2.5307, -44.3068, 1108975, false FROM countries c WHERE c.iso2 = 'BR'
UNION ALL SELECT c.id, 'São Gonçalo', 'Rio de Janeiro', -22.8268, -43.0539, 1091737, false FROM countries c WHERE c.iso2 = 'BR'
UNION ALL SELECT c.id, 'Maceió', 'Alagoas', -9.6658, -35.7353, 1025360, false FROM countries c WHERE c.iso2 = 'BR'
UNION ALL SELECT c.id, 'Duque de Caxias', 'Rio de Janeiro', -22.7858, -43.3117, 924624, false FROM countries c WHERE c.iso2 = 'BR'
UNION ALL SELECT c.id, 'Natal', 'Rio Grande do Norte', -5.7945, -35.2110, 890480, false FROM countries c WHERE c.iso2 = 'BR'
UNION ALL SELECT c.id, 'Teresina', 'Piauí', -5.0892, -42.8016, 868075, false FROM countries c WHERE c.iso2 = 'BR'
ON CONFLICT (id) DO NOTHING;

-- Australia - 20 cities
INSERT INTO cities (country_id, name, state_name, latitude, longitude, population, is_capital)
SELECT c.id, 'Adelaide', 'South Australia', -34.9285, 138.6007, 1345777, false FROM countries c WHERE c.iso2 = 'AU'
UNION ALL SELECT c.id, 'Gold Coast', 'Queensland', -28.0167, 153.4000, 679127, false FROM countries c WHERE c.iso2 = 'AU'
UNION ALL SELECT c.id, 'Newcastle', 'New South Wales', -32.9283, 151.7817, 322278, false FROM countries c WHERE c.iso2 = 'AU'
UNION ALL SELECT c.id, 'Wollongong', 'New South Wales', -34.4278, 150.8931, 302739, false FROM countries c WHERE c.iso2 = 'AU'
UNION ALL SELECT c.id, 'Hobart', 'Tasmania', -42.8821, 147.3272, 240342, false FROM countries c WHERE c.iso2 = 'AU'
UNION ALL SELECT c.id, 'Geelong', 'Victoria', -38.1499, 144.3617, 282809, false FROM countries c WHERE c.iso2 = 'AU'
UNION ALL SELECT c.id, 'Townsville', 'Queensland', -19.2590, 146.8169, 180820, false FROM countries c WHERE c.iso2 = 'AU'
UNION ALL SELECT c.id, 'Cairns', 'Queensland', -16.9186, 145.7781, 152729, false FROM countries c WHERE c.iso2 = 'AU'
UNION ALL SELECT c.id, 'Darwin', 'Northern Territory', -12.4634, 130.8456, 146245, false FROM countries c WHERE c.iso2 = 'AU'
UNION ALL SELECT c.id, 'Toowoomba', 'Queensland', -27.5598, 151.9507, 138468, false FROM countries c WHERE c.iso2 = 'AU'
UNION ALL SELECT c.id, 'Ballarat', 'Victoria', -37.5622, 143.8503, 105471, false FROM countries c WHERE c.iso2 = 'AU'
UNION ALL SELECT c.id, 'Bendigo', 'Victoria', -36.7570, 144.2794, 103034, false FROM countries c WHERE c.iso2 = 'AU'
UNION ALL SELECT c.id, 'Albury', 'New South Wales', -36.0737, 146.9135, 53677, false FROM countries c WHERE c.iso2 = 'AU'
UNION ALL SELECT c.id, 'Launceston', 'Tasmania', -41.4332, 147.1441, 87328, false FROM countries c WHERE c.iso2 = 'AU'
UNION ALL SELECT c.id, 'Mackay', 'Queensland', -21.1411, 149.1861, 80148, false FROM countries c WHERE c.iso2 = 'AU'
ON CONFLICT (id) DO NOTHING;

-- Mexico - 20 cities
INSERT INTO cities (country_id, name, state_name, latitude, longitude, population, is_capital)
SELECT c.id, 'Guadalajara', 'Jalisco', 20.6597, -103.3496, 1495189, false FROM countries c WHERE c.iso2 = 'MX'
UNION ALL SELECT c.id, 'Monterrey', 'Nuevo León', 25.6866, -100.3161, 1135512, false FROM countries c WHERE c.iso2 = 'MX'
UNION ALL SELECT c.id, 'Puebla', 'Puebla', 19.0414, -98.2063, 1576259, false FROM countries c WHERE c.iso2 = 'MX'
UNION ALL SELECT c.id, 'Tijuana', 'Baja California', 32.5149, -117.0382, 1810645, false FROM countries c WHERE c.iso2 = 'MX'
UNION ALL SELECT c.id, 'León', 'Guanajuato', 21.1212, -101.6800, 1578626, false FROM countries c WHERE c.iso2 = 'MX'
UNION ALL SELECT c.id, 'Juárez', 'Chihuahua', 31.6904, -106.4245, 1512354, false FROM countries c WHERE c.iso2 = 'MX'
UNION ALL SELECT c.id, 'Zapopan', 'Jalisco', 20.7214, -103.4005, 1332272, false FROM countries c WHERE c.iso2 = 'MX'
UNION ALL SELECT c.id, 'Mérida', 'Yucatán', 20.9674, -89.5926, 892363, false FROM countries c WHERE c.iso2 = 'MX'
UNION ALL SELECT c.id, 'Cancún', 'Quintana Roo', 21.1619, -86.8515, 888797, false FROM countries c WHERE c.iso2 = 'MX'
UNION ALL SELECT c.id, 'Aguascalientes', 'Aguascalientes', 21.8853, -102.2916, 934424, false FROM countries c WHERE c.iso2 = 'MX'
UNION ALL SELECT c.id, 'Querétaro', 'Querétaro', 20.5888, -100.3899, 1049777, false FROM countries c WHERE c.iso2 = 'MX'
UNION ALL SELECT c.id, 'Mexicali', 'Baja California', 32.6519, -115.4683, 1049364, false FROM countries c WHERE c.iso2 = 'MX'
UNION ALL SELECT c.id, 'Saltillo', 'Coahuila', 25.4260, -100.9737, 864431, false FROM countries c WHERE c.iso2 = 'MX'
UNION ALL SELECT c.id, 'Hermosillo', 'Sonora', 29.0729, -110.9559, 884273, false FROM countries c WHERE c.iso2 = 'MX'
UNION ALL SELECT c.id, 'Toluca', 'México', 19.2827, -99.6557, 910608, false FROM countries c WHERE c.iso2 = 'MX'
ON CONFLICT (id) DO NOTHING;

-- ============================================================================
-- PART 2: CREATE SAMPLE SUPPLIERS FOR CITIES
-- ============================================================================

-- This will create 3-5 suppliers per city that has been added
-- Using a stored procedure for efficiency

DO $$
DECLARE
    city_record RECORD;
    supplier_names TEXT[] := ARRAY[
        'Global Supplies Co.', 'Tech Solutions Ltd.', 'Premium Traders Inc.',
        'Industrial Parts Corp.', 'Metro Distributors', 'Elite Wholesale LLC',
        'Pacific Trading Company', 'Continental Exports', 'Regional Suppliers',
        'Quality Goods Ltd.', 'Express Logistics Co.', 'Prime Vendors Inc.',
        'International Trade Group', 'Unified Wholesalers', 'Direct Supply Chain',
        'Mega Distributors', 'Swift Trading Partners', 'Reliable Resources Ltd.'
    ];
    random_name TEXT;
    supplier_count INTEGER;
    i INTEGER;
    default_tenant_id UUID;
BEGIN
    -- Get the default tenant ID (first tenant)
    SELECT id INTO default_tenant_id FROM tenants LIMIT 1;

    -- For each city, create 3-5 random suppliers
    FOR city_record IN
        SELECT c.id, c.name, c.country_id, co.name as country_name
        FROM cities c
        JOIN countries co ON c.country_id = co.id
        ORDER BY c.id
    LOOP
        -- Random number between 3 and 5
        supplier_count := 3 + floor(random() * 3)::integer;

        FOR i IN 1..supplier_count LOOP
            -- Pick a random supplier name
            random_name := supplier_names[1 + floor(random() * array_length(supplier_names, 1))::integer];

            -- Create supplier
            INSERT INTO suppliers (id, tenant_id, name, contact_name, email, phone, address, city_id, created_at, updated_at)
            VALUES (
                uuid_generate_v4(),
                default_tenant_id,
                random_name || ' - ' || city_record.name,
                'Contact Person ' || i,
                lower(replace(random_name, ' ', '')) || i || '@' || lower(replace(city_record.name, ' ', '')) || '.com',
                '+' || (floor(random() * 999) + 1)::text || '-' || (floor(random() * 9000000) + 1000000)::text,
                (floor(random() * 9999) + 1)::text || ' Main Street, ' || city_record.name || ', ' || city_record.country_name,
                city_record.id,
                NOW(),
                NOW()
            );
        END LOOP;
    END LOOP;

    RAISE NOTICE 'Created suppliers for all cities!';
END $$;

-- ============================================================================
-- VERIFICATION
-- ============================================================================

-- Show summary of what was created
DO $$
DECLARE
    region_count INTEGER;
    country_count INTEGER;
    city_count INTEGER;
    supplier_count INTEGER;
    cities_with_suppliers INTEGER;
BEGIN
    SELECT COUNT(*) INTO region_count FROM regions;
    SELECT COUNT(*) INTO country_count FROM countries;
    SELECT COUNT(*) INTO city_count FROM cities;
    SELECT COUNT(*) INTO supplier_count FROM suppliers WHERE city_id IS NOT NULL;
    SELECT COUNT(DISTINCT city_id) INTO cities_with_suppliers FROM suppliers WHERE city_id IS NOT NULL;

    RAISE NOTICE '==============================================';
    RAISE NOTICE 'DATABASE SEEDING COMPLETE';
    RAISE NOTICE '==============================================';
    RAISE NOTICE 'Regions:                   %', region_count;
    RAISE NOTICE 'Countries:                 %', country_count;
    RAISE NOTICE 'Cities:                    %', city_count;
    RAISE NOTICE 'Suppliers (with cities):   %', supplier_count;
    RAISE NOTICE 'Cities with suppliers:     %', cities_with_suppliers;
    RAISE NOTICE '==============================================';
    RAISE NOTICE 'Your globe should now show % markers!', cities_with_suppliers;
    RAISE NOTICE '==============================================';
END $$;
