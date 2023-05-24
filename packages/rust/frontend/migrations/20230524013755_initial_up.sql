-- Add migration script here
CREATE TABLE "user" (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    identity TEXT NOT NULL,
    updated_date TEXT NOT NULL
);

CREATE TABLE client (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT NOT NULL UNIQUE,
    user_id INTEGER NOT NULL,
    signing_key TEXT NOT NULL,
    validated_date TEXT NULL,
    FOREIGN KEY (user_id) REFERENCES user (id)
);

CREATE TABLE location (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    client_id INTEGER NOT NULL,
    longitude REAL NOT NULL,
    latitude REAL NOT NULL,
    raw TEXT NOT NULL,
    created_date TEXT NOT NULL,
    group_uuid TEXT NOT NULL,
    FOREIGN KEY (client_id) REFERENCES client (id)
);

CREATE TABLE kv (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    "key" TEXT NOT NULL UNIQUE,
    "value" TEXT NOT NULL
);

CREATE TABLE keystore (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    "key" TEXT NOT NULL UNIQUE,
    "value" TEXT NOT NULL UNIQUE
);