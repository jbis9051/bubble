DROP TABLE key_package;

DROP TABLE recipient;

DROP TABLE message;

ALTER TABLE
    "user" DROP CONSTRAINT fk_user_primary_client_id;

DROP TABLE client;

DROP TABLE "session";

DROP TABLE confirmation;

DROP TABLE forgot;

DROP TABLE "user";