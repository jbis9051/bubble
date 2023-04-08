CREATE TABLE "user"
(
    id              serial PRIMARY KEY,
    uuid            UUID UNIQUE         NOT NULL,
    username        VARCHAR(255) UNIQUE NOT NULL,
    password        VARCHAR(255)            NULL,
    profile_picture VARCHAR(255) NULL,
    email           VARCHAR(255) UNIQUE NULL,
    phone           VARCHAR(11) UNIQUE NULL,
    name            VARCHAR(255)        NOT NULL,
    created         TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted         TIMESTAMP NULL
);

CREATE TABLE location
(
    id        serial PRIMARY KEY,
    user_id   INT REFERENCES "user" (id) NOT NULL,
    latitude  FLOAT(10)                  NOT NULL,
    longitude FLOAT(10)                  NOT NULL,
    created   TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE "group"
(
    id         serial PRIMARY KEY,
    uuid       UUID UNIQUE  NOT NULL,
    group_name VARCHAR(255) NOT NULL,
    created    TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE location_group
(
    id          serial PRIMARY KEY,
    location_id INT REFERENCES location (id) NOT NULL,
    group_id    INT REFERENCES "group" (id)  NOT NULL,
    created     TIMESTAMP DEFAULT CURRENT_TIMESTAMP

);

CREATE TABLE attachment
(
    id      serial PRIMARY KEY,
    "file"  VARCHAR(255) NOT NULL,
    created TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE member
(
    id       serial PRIMARY KEY,
    user_id  INT REFERENCES "user" (id)  NOT NULL,
    group_id INT REFERENCES "group" (id) NOT NULL,
    role_id  INTEGER                     NOT NULL,
    created  TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (user_id, group_id)
);


CREATE TABLE message
(
    id            serial PRIMARY KEY,
    attachment_id INT REFERENCES attachment (id) NULL,
    group_id      INT REFERENCES "group" (id) NOT NULL,
    user_id       INT REFERENCES "user" (id)  NOT NULL,
    content       VARCHAR(255) NULL,
    created       TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE confirmation
(
    id      serial PRIMARY KEY,
    user_id INT REFERENCES "user" (id) NOT NULL,
    link_id UUID UNIQUE                NOT NULL,
    email   VARCHAR(255)               NOT NULL,
    created TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE forgot_password
(
    id        SERIAL PRIMARY KEY,
    user_id   INT REFERENCES "user" (id) NOT NULL,
    forgot_id UUID UNIQUE                NOT NULL,
    created   TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE session
(
    id      SERIAL PRIMARY KEY,
    user_id INT REFERENCES "user" (id) NOT NULL,
    token   UUID UNIQUE                NOT NULL,
    created TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)

