
-- Undo: Create trigger for the Views table
DROP TRIGGER update_views_updated_at on views;

-- Undo: Create function to automatically update the `updated_at` column with a timestamp
DROP FUNCTION update_timestamp;

-- Undo: Create the Views table
DROP TABLE views;

-- Undo: Create the Visibility types enum
DROP TYPE visibility_types;

-- Undo: Enable the uuid-ossp extension for generating UUIDs
DROP EXTENSION "uuid-ossp";
