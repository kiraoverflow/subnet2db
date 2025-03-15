-- Your SQL goes here

-- Create subnet table
CREATE TABLE subnet (
    id SERIAL PRIMARY KEY,
    target_id INT NOT NULL REFERENCES target(id),
    cidr VARCHAR(255) NOT NULL UNIQUE
);

