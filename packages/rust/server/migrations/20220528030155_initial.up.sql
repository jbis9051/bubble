CREATE TABLE "user"
(
    id              serial PRIMARY KEY,
    username        VARCHAR(255) UNIQUE NOT NULL,
    password        VARCHAR(255)        NOT NULL,
    profile_picture VARCHAR(255)        NULL,
    email           VARCHAR(255) UNIQUE NULL,
    phone           VARCHAR(11) UNIQUE  NOT NULL,
    name            VARCHAR(255)        NOT NULL,
    created         TIMESTAMP           NOT NULL
);

CREATE TABLE location
(
    id        serial PRIMARY KEY,
    user_id   INT REFERENCES "user" (id) NOT NULL,
    latitude  FLOAT(10)                  NOT NULL,
    longitude FLOAT(10)                  NOT NULL,
    created   TIMESTAMP                  NOT NULL
);

CREATE TABLE "group"
(
    id         serial PRIMARY KEY,
    group_name VARCHAR(255) NOT NULL,
    created    TIMESTAMP    NOT NULL
);

CREATE TABLE location_group
(
    id          serial PRIMARY KEY,
    location_id INT REFERENCES location (id) NOT NULL,
    group_id    INT REFERENCES "group" (id)  NOT NULL,
    created     TIMESTAMP                    NOT NULL

);

CREATE TABLE role
(
    id      serial PRIMARY KEY,
    role    VARCHAR(255) NOT NULL,
    created TIMESTAMP    NOT NULL
);

CREATE TABLE attachment
(
    id      serial PRIMARY KEY,
    "file"  VARCHAR(255) NOT NULL,
    created TIMESTAMP    NOT NULL
);

CREATE TABLE user_group
(
    id       serial PRIMARY KEY,
    user_id  INT REFERENCES "user" (id)  NOT NULL,
    group_id INT REFERENCES "group" (id) NOT NULL,
    role_id  INT REFERENCES role (id)    NOT NULL,
    created  TIMESTAMP                   NOT NULL
);


CREATE TABLE message
(
    id            serial PRIMARY KEY,
    attachment_id INT REFERENCES attachment (id) NOT NULL,
    group_id      INT REFERENCES "group" (id)    NOT NULL,
    user_id       INT REFERENCES "user" (id)     NOT NULL,
    content       VARCHAR(255)                   NULL,
    created       TIMESTAMP                      NOT NULL
);


