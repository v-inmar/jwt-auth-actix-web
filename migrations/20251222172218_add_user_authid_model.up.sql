-- Add up migration script here
CREATE TABLE user_authid_model (
    id SERIAL PRIMARY KEY,
    value VARCHAR(255) NOT NULL UNIQUE,
    datetime_created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP -- default postgresql timezone is utc, this will be automatically filled in
);