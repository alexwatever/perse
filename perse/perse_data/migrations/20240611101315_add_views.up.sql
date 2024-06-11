
-- Enable the uuid-ossp extension for generating UUIDs
CREATE EXTENSION "uuid-ossp";

-- Create the Visibility types enum
CREATE TYPE visibility_types AS ENUM (
    'VisibilityPublic',
    'VisibilityUnlisted',
    'VisibilityHidden'
);

-- Create the Views table
CREATE TABLE views (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(), -- Using UUID as the primary key
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    visibility visibility_types NOT NULL,
    title VARCHAR(255) NOT NULL,
    content_body TEXT,
    content_head TEXT,

    description VARCHAR(255),
    route VARCHAR(255) NOT NULL UNIQUE
);

-- Create indexes
CREATE INDEX idx_views_route ON views (route);

-- Create function to automatically update the `updated_at` column with a timestamp
CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
   NEW.updated_at = CURRENT_TIMESTAMP;
   RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger for the Views table
CREATE TRIGGER update_views_updated_at
BEFORE UPDATE ON views
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();
