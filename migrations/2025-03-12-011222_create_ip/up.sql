-- Your SQL goes here
-- Create ip table
CREATE TABLE ip (
    id SERIAL PRIMARY KEY,
    subnet_id INT NOT NULL REFERENCES subnet(id),
    v4 VARCHAR(255) UNIQUE NULL,
    v6 VARCHAR(255) UNIQUE NULL
);
