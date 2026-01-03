-- Add up migration script here
CREATE TABLE user_name_model (
    id SERIAL PRIMARY KEY,
    value VARCHAR(128) NOT NULL UNIQUE,
    datetime_created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP -- default postgresql timezone is utc, this will be automatically filled in
);