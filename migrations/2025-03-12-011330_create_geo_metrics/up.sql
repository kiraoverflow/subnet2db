-- Create geo_metrics table
CREATE TABLE geo_metrics (
    id SERIAL PRIMARY KEY,
    ip_id INT NOT NULL REFERENCES ip(id),
    city VARCHAR(255) NOT NULL UNIQUE,
    hops_to_turkey VARCHAR(255) UNIQUE NULL,
    hops_to_iraq VARCHAR(255) UNIQUE NULL,
    hops_to_jordan VARCHAR(255) UNIQUE NULL,
    hops_to_lebanon VARCHAR(255) UNIQUE NULL,
    hops_to_israel VARCHAR(255) UNIQUE NULL
);
