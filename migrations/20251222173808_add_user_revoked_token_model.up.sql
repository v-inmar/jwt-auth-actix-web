-- Add up migration script here
CREATE TABLE user_revoked_token_model (
    id SERIAL PRIMARY KEY,
    value VARCHAR(512) NOT NULL,
    datetime_ttl TIMESTAMP WITH TIME ZONE NOT NULL,
    datetime_created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);