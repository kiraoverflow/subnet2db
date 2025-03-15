-- Your SQL goes here
-- Create port table
CREATE TABLE port (
    id SERIAL PRIMARY KEY,
    ip_id INT NOT NULL REFERENCES ip(id),
    number INT NOT NULL UNIQUE,
    service VARCHAR(255) NULL,
    timeout BOOLEAN NULL,
    answer BOOLEAN NULL,
    deny BOOLEAN NULL
);
