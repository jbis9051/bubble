CREATE TABLE user (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    identity TEXT NOT NULL,
    updated_date INTEGER NOT NULL
);

CREATE TABLE client (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT NOT NULL UNIQUE,
    user_uuid TEXT NOT NULL,
    signing_key TEXT NOT NULL,
    validated_date INTEGER NULL,
    created_date INTEGER NULL
);

CREATE TABLE location (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    client_uuid TEXT NOT NULL,
    group_uuid TEXT NOT NULL,
    longitude REAL NOT NULL,
    latitude REAL NOT NULL,
    location_date INTEGER NOT NULL,
    raw BLOB NOT NULL,
    created_date INTEGER NOT NULL
);

CREATE TABLE "group" (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT NOT NULL UNIQUE,
    name TEXT NULL,
    image BLOB NULL
);

CREATE TABLE kv (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    "key" TEXT NOT NULL UNIQUE,
    value TEXT NOT NULL,
    created_date INTEGER NOT NULL
);

CREATE TABLE keystore (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    "key" BLOB NOT NULL UNIQUE,
    value BLOB NOT NULL UNIQUE,
    type_name  TEXT NOT NULL,
    created_date INTEGER NOT NULL
);