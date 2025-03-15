-- Create webserver table
CREATE TABLE webserver (
    id SERIAL PRIMARY KEY,
    ip_id INT NOT NULL REFERENCES ip(id),
    port_id INT NOT NULL REFERENCES port(id),
    crawer_id INT NOT NULL REFERENCES crawler(id),
    reachable BOOLEAN NULL,
    framework VARCHAR(255) NULL
);
