-- Create endpoint table
CREATE TABLE endpoint (
    id SERIAL PRIMARY KEY,
    route VARCHAR(255) UNIQUE NULL,
    status_code INT NULL,
    credentials BOOLEAN NULL,
    authentification BOOLEAN NULL,
    port_id INT NOT NULL REFERENCES port(id),
    ip_id INT NOT NULL REFERENCES ip(id)
);
