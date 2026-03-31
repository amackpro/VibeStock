
# VibeStock Showcase Seed - Fixed version
$psql = "C:\Program Files\PostgreSQL\18\bin\psql.exe"
$db   = "postgresql://postgres:postgres@localhost:5432/vibestock"

function Q($sql) {
    $bytes = [System.Text.Encoding]::UTF8.GetBytes($sql)
    $tmp = [System.IO.Path]::GetTempFileName() -replace '\.tmp$','.sql'
    [System.IO.File]::WriteAllBytes($tmp, $bytes)
    $out = & $psql $db -f $tmp 2>&1
    Remove-Item $tmp -ErrorAction SilentlyContinue
    return $out
}

function QV($sql) {
    $bytes = [System.Text.Encoding]::UTF8.GetBytes($sql)
    $tmp = [System.IO.Path]::GetTempFileName() -replace '\.tmp$','.sql'
    [System.IO.File]::WriteAllBytes($tmp, $bytes)
    $val = & $psql $db -t -A -f $tmp 2>&1
    Remove-Item $tmp -ErrorAction SilentlyContinue
    return $val.Trim()
}

Write-Host "== Getting tenant + user ==" -ForegroundColor Cyan
$tenantId = QV "SELECT id FROM tenants WHERE slug='default' LIMIT 1;"
$adminId  = QV "SELECT id FROM users WHERE username='admin' LIMIT 1;"
Write-Host "Tenant=$tenantId  Admin=$adminId"
if (-not $tenantId -or $tenantId -like "*ERROR*") { Write-Error "No tenant found! Run API with RESET_DB=true first."; exit 1 }

Write-Host "== Categories ==" -ForegroundColor Cyan
Q "INSERT INTO categories (name, description, tenant_id) VALUES
  ('Electronics', 'Phones, Laptops, Gadgets', '$tenantId'),
  ('Stationery',  'Office and school supplies', '$tenantId'),
  ('Medical',     'Healthcare and medical equipment', '$tenantId')
ON CONFLICT (name) DO UPDATE SET tenant_id = EXCLUDED.tenant_id;" | Out-Null

$catElec = QV "SELECT id FROM categories WHERE name='Electronics' LIMIT 1;"
$catStat = QV "SELECT id FROM categories WHERE name='Stationery' LIMIT 1;"
$catMed  = QV "SELECT id FROM categories WHERE name='Medical' LIMIT 1;"
Write-Host "Elec=$catElec Stat=$catStat Med=$catMed"

Write-Host "== Regions ==" -ForegroundColor Cyan
Q "INSERT INTO regions (name, code, latitude, longitude) VALUES
  ('Asia',          'AS',  34.0,   100.0),
  ('Europe',        'EU',  54.0,   15.0),
  ('North America', 'NA',  45.0,  -100.0),
  ('South America', 'SA', -15.0,  -60.0),
  ('Africa',        'AF',   2.0,   22.0),
  ('Oceania',       'OC', -25.0,  135.0)
ON CONFLICT (code) DO NOTHING;" | Out-Null

$rAS = QV "SELECT id FROM regions WHERE code='AS';"
$rEU = QV "SELECT id FROM regions WHERE code='EU';"
$rNA = QV "SELECT id FROM regions WHERE code='NA';"
$rSA = QV "SELECT id FROM regions WHERE code='SA';"
$rAF = QV "SELECT id FROM regions WHERE code='AF';"
$rOC = QV "SELECT id FROM regions WHERE code='OC';"

Write-Host "== Countries (3 per region) ==" -ForegroundColor Cyan
Q "INSERT INTO countries (region_id, name, iso2, iso3, phone_code, capital, latitude, longitude) VALUES
  ('$rAS','India',           'IN','IND','+91', 'New Delhi',    20.59,  78.96),
  ('$rAS','China',           'CN','CHN','+86', 'Beijing',      35.86, 104.19),
  ('$rAS','Japan',           'JP','JPN','+81', 'Tokyo',        36.20, 138.25),
  ('$rEU','Germany',         'DE','DEU','+49', 'Berlin',       51.16,  10.45),
  ('$rEU','France',          'FR','FRA','+33', 'Paris',        46.22,   2.21),
  ('$rEU','United Kingdom',  'GB','GBR','+44', 'London',       55.37,  -3.43),
  ('$rNA','United States',   'US','USA','+1',  'Washington',   37.09, -95.71),
  ('$rNA','Canada',          'CA','CAN','+1',  'Ottawa',       56.13,-106.34),
  ('$rNA','Mexico',          'MX','MEX','+52', 'Mexico City',  23.63,-102.55),
  ('$rSA','Brazil',          'BR','BRA','+55', 'Brasilia',    -14.23, -51.92),
  ('$rSA','Argentina',       'AR','ARG','+54', 'Buenos Aires',-38.41, -63.61),
  ('$rSA','Colombia',        'CO','COL','+57', 'Bogota',        4.57, -74.29),
  ('$rAF','Nigeria',         'NG','NGA','+234','Abuja',          9.08,   8.67),
  ('$rAF','South Africa',    'ZA','ZAF','+27', 'Pretoria',    -30.55,  22.93),
  ('$rAF','Egypt',           'EG','EGY','+20', 'Cairo',        26.82,  30.80),
  ('$rOC','Australia',       'AU','AUS','+61', 'Canberra',    -25.27, 133.77),
  ('$rOC','New Zealand',     'NZ','NZL','+64', 'Wellington',  -40.90, 174.88),
  ('$rOC','Papua New Guinea','PG','PNG','+675','Port Moresby', -6.31, 143.95)
ON CONFLICT (iso2) DO NOTHING;" | Out-Null
Write-Host "Countries inserted."

# City data: iso2 -> array of [name, state, lat, lon, pop]
$cityData = @{
  IN=@(@("Mumbai","Maharashtra",19.07,72.87,20600000),@("Delhi","Delhi",28.70,77.10,31000000),@("Bangalore","Karnataka",12.97,77.59,12000000),@("Hyderabad","Telangana",17.38,78.47,9500000),@("Chennai","Tamil Nadu",13.08,80.27,8600000),@("Kolkata","West Bengal",22.57,88.36,14600000),@("Pune","Maharashtra",18.52,73.85,6200000))
  CN=@(@("Shanghai","Shanghai",31.22,121.47,24000000),@("Beijing","Beijing",39.90,116.40,21500000),@("Guangzhou","Guangdong",23.12,113.26,16000000),@("Shenzhen","Guangdong",22.54,114.06,13000000),@("Chengdu","Sichuan",30.57,104.07,9000000),@("Wuhan","Hubei",30.59,114.30,11000000),@("Hangzhou","Zhejiang",30.25,120.15,7200000))
  JP=@(@("Tokyo","Tokyo",35.68,139.69,13960000),@("Osaka","Osaka",34.69,135.50,2691000),@("Nagoya","Aichi",35.18,136.90,2296000),@("Sapporo","Hokkaido",43.06,141.35,1973000),@("Fukuoka","Fukuoka",33.59,130.40,1612000),@("Kobe","Hyogo",34.69,135.19,1544000),@("Kyoto","Kyoto",35.01,135.76,1474000))
  DE=@(@("Berlin","Berlin",52.52,13.40,3769000),@("Hamburg","Hamburg",53.57,10.01,1845000),@("Munich","Bavaria",48.13,11.57,1471000),@("Frankfurt","Hesse",50.11,8.68,753000),@("Cologne","NRW",50.93,6.96,1084000),@("Stuttgart","BW",48.77,9.18,636000),@("Dusseldorf","NRW",51.22,6.77,617000))
  FR=@(@("Paris","Ile-de-France",48.85,2.35,2161000),@("Lyon","Auvergne",45.75,4.83,522000),@("Marseille","PACA",43.29,5.38,861000),@("Toulouse","Occitanie",43.60,1.44,479000),@("Nice","PACA",43.71,7.26,342000),@("Bordeaux","NAqt",44.83,-0.57,257000),@("Strasbourg","Grand Est",48.57,7.75,283000))
  GB=@(@("London","England",51.51,-0.12,8982000),@("Birmingham","England",52.48,-1.89,1141000),@("Manchester","England",53.48,-2.24,553000),@("Glasgow","Scotland",55.86,-4.25,633000),@("Leeds","England",53.80,-1.54,793000),@("Liverpool","England",53.41,-2.99,498000),@("Edinburgh","Scotland",55.95,-3.18,524000))
  US=@(@("New York","New York",40.71,-74.00,8336817),@("Los Angeles","California",34.05,-118.24,3979576),@("Chicago","Illinois",41.85,-87.65,2693976),@("Houston","Texas",29.76,-95.36,2304580),@("Phoenix","Arizona",33.44,-112.07,1608139),@("Dallas","Texas",32.78,-96.80,1345047),@("Seattle","Washington",47.60,-122.33,737255))
  CA=@(@("Toronto","Ontario",43.65,-79.38,2930000),@("Vancouver","BC",49.25,-123.12,675218),@("Montreal","Quebec",45.50,-73.57,1762949),@("Calgary","Alberta",51.04,-114.07,1336000),@("Edmonton","Alberta",53.55,-113.46,981280),@("Ottawa","Ontario",45.42,-75.69,994837),@("Winnipeg","Manitoba",49.89,-97.14,749534))
  MX=@(@("Mexico City","CDMX",19.43,-99.13,8918653),@("Guadalajara","Jalisco",20.65,-103.34,1495182),@("Monterrey","NL",25.69,-100.31,1135512),@("Puebla","Puebla",19.04,-98.20,1692181),@("Tijuana","BC",32.53,-117.04,1925000),@("Leon","Guanajuato",21.12,-101.68,1579803),@("Juarez","Chihuahua",31.73,-106.48,1501000))
  BR=@(@("Sao Paulo","SP",-23.55,-46.63,12325232),@("Rio de Janeiro","RJ",-22.91,-43.17,6748000),@("Brasilia","DF",-15.78,-47.92,3055149),@("Salvador","Bahia",-12.97,-38.50,2900319),@("Fortaleza","Ceara",-3.72,-38.54,2669342),@("Belo Horizonte","MG",-19.92,-43.93,2521564),@("Manaus","AM",-3.10,-60.02,2219580))
  AR=@(@("Buenos Aires","BA",-34.61,-58.37,2890151),@("Cordoba","Cordoba",-31.42,-64.18,1391000),@("Rosario","Santa Fe",-32.94,-60.65,948312),@("Mendoza","Mendoza",-32.89,-68.84,115041),@("Tucuman","Tucuman",-26.82,-65.22,548866),@("La Plata","BA",-34.92,-57.95,899000),@("Mar del Plata","BA",-38.00,-57.56,614350))
  CO=@(@("Bogota","Cundinamarca",4.71,-74.07,7181469),@("Medellin","Antioquia",6.25,-75.56,2529403),@("Cali","Valle",3.43,-76.52,2401000),@("Barranquilla","Atlantico",11.00,-74.80,1228621),@("Cartagena","Bolivar",10.39,-75.51,1028736),@("Bucaramanga","Santander",7.12,-73.11,577353),@("Pereira","Risaralda",4.81,-75.69,472000))
  NG=@(@("Lagos","Lagos",6.45,3.39,14862000),@("Kano","Kano",12.00,8.52,3626068),@("Ibadan","Oyo",7.38,3.90,3565108),@("Abuja","FCT",9.07,7.39,2440000),@("Port Harcourt","Rivers",4.77,7.01,1865000),@("Benin City","Edo",6.34,5.61,1496000),@("Maiduguri","Borno",11.83,13.15,1112000))
  ZA=@(@("Johannesburg","Gauteng",-26.20,28.04,5635127),@("Cape Town","Western Cape",-33.93,18.42,4618000),@("Durban","KZN",-29.85,31.01,3720000),@("Pretoria","Gauteng",-25.74,28.18,2921000),@("Port Elizabeth","EC",-33.96,25.60,1152115),@("Bloemfontein","FS",-29.12,26.21,256185),@("East London","EC",-33.01,27.91,478676))
  EG=@(@("Cairo","Cairo",30.06,31.24,10100000),@("Alexandria","Alexandria",31.20,29.92,5200000),@("Giza","Giza",30.01,31.21,3628062),@("Luxor","Luxor",25.69,32.64,422407),@("Aswan","Aswan",24.09,32.90,290000),@("Helwan","Cairo",29.84,31.33,652000),@("Shubra","Cairo",30.12,31.24,1099354))
  AU=@(@("Sydney","NSW",-33.87,151.21,5312000),@("Melbourne","Victoria",-37.81,144.96,5078000),@("Brisbane","Queensland",-27.47,153.02,2514000),@("Perth","WA",-31.95,115.86,2059000),@("Adelaide","SA",-34.93,138.60,1376000),@("Canberra","ACT",-35.28,149.13,453000),@("Gold Coast","Queensland",-28.00,153.43,679000))
  NZ=@(@("Auckland","Auckland",-36.86,174.76,1669000),@("Wellington","Wellington",-41.28,174.77,215000),@("Christchurch","Canterbury",-43.53,172.63,375000),@("Hamilton","Waikato",-37.78,175.28,169000),@("Tauranga","Bay of Plenty",-37.68,176.17,155000),@("Napier","Hawkes Bay",-39.49,176.91,65000),@("Dunedin","Otago",-45.87,170.50,130000))
  PG=@(@("Port Moresby","NCD",-9.44,147.18,365000),@("Lae","Morobe",-6.72,147.00,148000),@("Mt Hagen","WHP",-5.86,144.22,46000),@("Madang","Madang",-5.22,145.79,27420),@("Wewak","ESP",-3.55,143.63,25143),@("Goroka","EHP",-6.08,145.38,18300),@("Kokopo","ENB",-4.36,152.27,26300))
}

# Supplier/product templates per category
$catConfig = @(
  @{id=$catElec; name="Electronics"; sup1="TechZone";  sup2="DigiHub";
    prods=@(
      @{name="Laptop 15in";     sku="ELEC-LAP"; price=1200; cost=950;  qty=30;  uom="pcs"},
      @{name="Wireless Mouse";  sku="ELEC-MOU"; price=35;   cost=20;   qty=200; uom="pcs"},
      @{name="USB-C Hub";       sku="ELEC-USB"; price=55;   cost=35;   qty=150; uom="pcs"}
    )
  },
  @{id=$catStat; name="Stationery"; sup1="PaperWorld"; sup2="OfficeMax";
    prods=@(
      @{name="A4 Paper Ream";   sku="STAT-PAP"; price=12;   cost=7;    qty=500; uom="ream"},
      @{name="Ballpen x10";     sku="STAT-PEN"; price=5;    cost=2;    qty=1000;uom="pack"},
      @{name="Spiral Notebook"; sku="STAT-NB";  price=8;    cost=4;    qty=300; uom="pcs"}
    )
  },
  @{id=$catMed;  name="Medical";    sup1="MedSupply";  sup2="HealthStore";
    prods=@(
      @{name="Gloves 100pk";    sku="MED-GLV"; price=18;   cost=10;   qty=400; uom="box"},
      @{name="Digital Therm";   sku="MED-THM"; price=22;   cost=14;   qty=200; uom="pcs"},
      @{name="N95 Mask 50pk";   sku="MED-N95"; price=30;   cost=18;   qty=300; uom="box"}
    )
  }
)

$sc = 0; $pc = 0

foreach ($iso in $cityData.Keys) {
    $cId = QV "SELECT id FROM countries WHERE iso2='$iso';"
    if (-not $cId -or $cId -like "*ERROR*") { Write-Host "SKIP $iso" -ForegroundColor Red; continue }

    foreach ($city in $cityData[$iso]) {
        $cn = $city[0] -replace "'","''"
        $st = $city[1] -replace "'","''"
        $la = $city[2]; $lo = $city[3]; $po = $city[4]

        Q "INSERT INTO cities (country_id,name,state_name,latitude,longitude,population) VALUES ('$cId','$cn','$st',$la,$lo,$po) ON CONFLICT DO NOTHING;" | Out-Null
        $cityId = QV "SELECT id FROM cities WHERE country_id='$cId' AND name='$cn' LIMIT 1;"

        foreach ($cat in $catConfig) {
            foreach ($supPrefix in @($cat.sup1, $cat.sup2)) {
                $sc++
                $sn = "$supPrefix $cn" -replace "'","''"
                $em = ($sn -replace "[^a-zA-Z0-9]","").ToLower().Substring(0,[Math]::Min(30,$sn.Length)) + "@vibestock.demo"
                Q "INSERT INTO suppliers (name,contact_name,email,phone,address,tenant_id,city_id)
                   VALUES ('$sn','Manager','$em','+0-000-$("{0:D4}" -f ($sc % 9999))','$cn','$tenantId','$cityId')
                   ON CONFLICT DO NOTHING;" | Out-Null
                $supId = QV "SELECT id FROM suppliers WHERE name='$sn' AND tenant_id='$tenantId' LIMIT 1;"

                foreach ($p in $cat.prods) {
                    $pc++
                    $sku = "$($p.sku)-$iso-$("{0:D4}" -f $pc)"
                    $pn  = $p.name -replace "'","''"
                    $qty = $p.qty + ($pc % 50)
                    Q "INSERT INTO products (name,sku,category_id,supplier_id,unit_price,cost_price,quantity_in_stock,reorder_level,unit_of_measure,tenant_id)
                       VALUES ('$pn','$sku','$($cat.id)','$supId',$($p.price),$($p.cost),$qty,10,'$($p.uom)','$tenantId')
                       ON CONFLICT (sku) DO NOTHING;" | Out-Null
                    $prodId = QV "SELECT id FROM products WHERE sku='$sku' LIMIT 1;"
                    if ($prodId -and $prodId -notlike "*ERROR*") {
                        Q "INSERT INTO stock_movements (product_id,movement_type,quantity,reference,notes,performed_by,tenant_id)
                           VALUES ('$prodId','in',$qty,'PO-INIT-$pc','Initial stock','$adminId','$tenantId');" | Out-Null
                    }
                }
            }
        }
    }
    Write-Host "  $iso done | suppliers=$sc products=$pc" -ForegroundColor Green
}

Write-Host ""
Write-Host "ALL DONE! Suppliers=$sc  Products=$pc" -ForegroundColor Yellow
