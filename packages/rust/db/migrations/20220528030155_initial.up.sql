CREATE TABLE "user"
(
    id              SERIAL PRIMARY KEY,
    uuid            UUID UNIQUE         NOT NULL,
    username        VARCHAR(255) UNIQUE NOT NULL,
    password        VARCHAR(255)        NOT NULL,
    email           VARCHAR(255) UNIQUE NULL,
    name            VARCHAR(255)        NOT NULL,
    created         TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE forgot
(
    id      SERIAL PRIMARY KEY,
    user_id INT REFERENCES "user" (id) ON DELETE CASCADE NOT NULL,
    token   UUID UNIQUE                                  NOT NULL,
    created TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE confirmation
(
    id      SERIAL PRIMARY KEY,
    user_id INT REFERENCES "user" (id) ON DELETE CASCADE NOT NULL,
    token   UUID UNIQUE                                  NOT NULL,
    email   VARCHAR(255)                                 NOT NULL,
    created TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE session
(
    id      SERIAL PRIMARY KEY,
    user_id INT REFERENCES "user" (id) ON DELETE CASCADE NOT NULL,
    token   UUID UNIQUE                                  NOT NULL,
    created TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE client
(
    id      SERIAL PRIMARY KEY,
    user_id INT REFERENCES "user" (id) ON DELETE CASCADE NOT NULL,
    uuid    UUID UNIQUE                                  NOT NULL,
    created TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE message
(
    id      SERIAL PRIMARY KEY,
    message BYTEA NOT NULL,
    created TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE recipient
(
    id         SERIAL PRIMARY KEY,
    client_id  INT REFERENCES client (id) ON DELETE CASCADE  NOT NULL,
    message_id INT REFERENCES message (id) ON DELETE CASCADE NOT NULL,
    created    TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE key_package
(
    id          SERIAL PRIMARY KEY,
    client_id   INT REFERENCES client (id) NOT NULL,
    key_package BYTEA                      NOT NULL,
    created     TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);


