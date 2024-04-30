-- Drop the table if it already exists to ensure a clean slate
DROP TABLE IF EXISTS cities;

-- Create the table with the given schema
CREATE TABLE cities (
    id SERIAL PRIMARY KEY,
    department_code VARCHAR(5),
    insee_code VARCHAR(10),
    zip_code VARCHAR(10),
    name VARCHAR(50),
    lat NUMERIC(10, 8),
    lon NUMERIC(10, 8)
);
