CREATE TABLE IF NOT EXISTS "user"
(
    user_id         serial PRIMARY KEY,
    date_time       TIMESTAMP           NOT NULL,
    username        VARCHAR(255) UNIQUE NOT NULL,
    password        VARCHAR(255)        NOT NULL,
    profile_picture VARCHAR(255)        NULL,
    email           VARCHAR(255) UNIQUE NULL,
    phone           INT UNIQUE          NOT NULL,
    name            VARCHAR(255)        NOT NULL
);

CREATE TABLE IF NOT EXISTS location
(
    location_id serial PRIMARY KEY,
    user_id     serial REFERENCES "user" (user_id) NOT NULL,
    latitude    FLOAT(10)                          NOT NULL,
    longitude   FLOAT(10)                          NOT NULL,
    date_time   TIMESTAMP                          NOT NULL
);

CREATE TABLE IF NOT EXISTS "group"
(
    group_id   serial PRIMARY KEY,
    group_name VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS location_group
(
    location_id serial REFERENCES location (location_id) NOT NULL,
    group_id    serial REFERENCES "group" (group_id)     NOT NULL
);

CREATE TABLE IF NOT EXISTS role
(
    role_id serial PRIMARY KEY,
    role    VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS attachment
(
    attachment_id serial PRIMARY KEY,
    date_time     TIMESTAMP    NOT NULL,
    "file"        VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS user_group
(
    user_id   serial REFERENCES "user" (user_id)   NOT NULL,
    group_id  serial REFERENCES "group" (group_id) NOT NULL,
    role_id   serial REFERENCES role (role_id)     NOT NULL,
    date_time TIMESTAMP                            NOT NULL
);


CREATE TABLE IF NOT EXISTS message
(
    message_id    serial PRIMARY KEY,
    attachment_id serial REFERENCES attachment (attachment_id) NOT NULL,
    group_id      serial REFERENCES "group" (group_id)         NOT NULL,
    user_id       serial REFERENCES "user" (user_id)           NOT NULL,
    content       VARCHAR(255)                                 NULL,
    date_time     TIMESTAMP                                    NOT NULL
);


