-- Your SQL goes here

-- Create geo_metrics table
-- Create target table 
CREATE TABLE target (
    id SERIAL PRIMARY KEY,
    country VARCHAR(255) NOT NULL UNIQUE
    --city INT NULL REFERENCES geo_metrics(city)
);
