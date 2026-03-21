-- Migration: 005 - Geography Support
-- Description: Add global geography tables (regions, countries, cities) and city association to suppliers

-- Enable UUID extension if not already enabled
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Global geography tables (not tenant-specific)

CREATE TABLE regions (
    id              UUID          PRIMARY KEY DEFAULT uuid_generate_v4(),
    name            VARCHAR(100)  NOT NULL UNIQUE,
    code            VARCHAR(10)   NOT NULL UNIQUE,  -- 'EU', 'AS', 'NA', etc.
    latitude        FLOAT8        NOT NULL,
    longitude       FLOAT8        NOT NULL,
    description     TEXT,
    created_at      TIMESTAMPTZ   NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_regions_code ON regions(code);
CREATE INDEX idx_regions_coords ON regions(latitude, longitude);

CREATE TABLE countries (
    id              UUID          PRIMARY KEY DEFAULT uuid_generate_v4(),
    region_id       UUID          NOT NULL REFERENCES regions(id) ON DELETE CASCADE,
    name            VARCHAR(255)  NOT NULL,
    iso2            VARCHAR(2)    NOT NULL UNIQUE,
    iso3            VARCHAR(3)    NOT NULL UNIQUE,
    phone_code      VARCHAR(20),
    capital         VARCHAR(255),
    latitude        FLOAT8        NOT NULL,
    longitude       FLOAT8        NOT NULL,
    created_at      TIMESTAMPTZ   NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_countries_region ON countries(region_id);
CREATE INDEX idx_countries_iso2 ON countries(iso2);
CREATE INDEX idx_countries_coords ON countries(latitude, longitude);

CREATE TABLE cities (
    id              UUID          PRIMARY KEY DEFAULT uuid_generate_v4(),
    country_id      UUID          NOT NULL REFERENCES countries(id) ON DELETE CASCADE,
    name            VARCHAR(255)  NOT NULL,
    state_name      VARCHAR(255),
    latitude        FLOAT8        NOT NULL,
    longitude       FLOAT8        NOT NULL,
    population      BIGINT        DEFAULT 0,
    is_capital      BOOLEAN       NOT NULL DEFAULT false,
    created_at      TIMESTAMPTZ   NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_cities_country ON cities(country_id);
CREATE INDEX idx_cities_coords ON cities(latitude, longitude);
CREATE INDEX idx_cities_population ON cities(population DESC);
CREATE INDEX idx_cities_country_name ON cities(country_id, name);

-- Add city_id to suppliers table
ALTER TABLE suppliers ADD COLUMN city_id UUID REFERENCES cities(id) ON DELETE SET NULL;
CREATE INDEX idx_suppliers_city ON suppliers(city_id);
