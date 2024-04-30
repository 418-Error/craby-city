-- Drop the table if it already exists to ensure a clean slate
DROP TABLE IF EXISTS cities;

-- Create the table with the given schema
CREATE TABLE cities (
    id SERIAL PRIMARY KEY,
    department_code VARCHAR(255),
    insee_code VARCHAR(255),
    zip_code VARCHAR(255),
    name VARCHAR(255),
    lat FLOAT(10),
    lon FLOAT(10)
);
