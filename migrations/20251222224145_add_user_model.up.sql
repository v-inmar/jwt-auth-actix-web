-- Add up migration script here
CREATE TABLE user_model (
    id SERIAL PRIMARY KEY,
    firstname_id INT NOT NULL,
    lastname_id INT NOT NULL,
    email_id INT NOT NULL UNIQUE,
    pid_id INT NOT NULL UNIQUE,
    authid_id INT NOT NULL UNIQUE,
    password VARCHAR(512) NOT NULL,
    datetime_deleted TIMESTAMP WITH TIME ZONE DEFAULT NULL, -- soft delete
    datetime_deactivated TIMESTAMP WITH TIME ZONE DEFAULT NULL, -- deactivate account
    datetime_verified TIMESTAMP WITH TIME ZONE DEFAULT NULL, -- verify via email
    datetime_created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP, -- default postgresql timezone is utc, this will be automatically filled in
    FOREIGN KEY (firstname_id) REFERENCES user_name_model(id),
    FOREIGN KEY (lastname_id) REFERENCES user_name_model(id),
    FOREIGN KEY (email_id) REFERENCES user_email_model(id),
    FOREIGN KEY (pid_id) REFERENCES user_pid_model(id),
    FOREIGN KEY (authid_id) REFERENCES user_authid_model(id)
);