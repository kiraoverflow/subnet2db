-- Create crawler table
CREATE TABLE crawler (
    id SERIAL PRIMARY KEY,
    ip_id INT NOT NULL REFERENCES ip(id),
    endpoint_id INT NOT NULL REFERENCES endpoint(id)
);
