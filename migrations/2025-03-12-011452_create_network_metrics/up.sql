-- Your SQL goes here
-- Create network_metrics table
CREATE TABLE network_metrics (
    id SERIAL PRIMARY KEY,
    ip_id INT NOT NULL REFERENCES ip(id),
    pingable BOOLEAN NULL,
    reachable BOOLEAN NULL,
    unreachable BOOLEAN NULL,
    ttr FLOAT NULL,
    ttl FLOAT NULL
);
