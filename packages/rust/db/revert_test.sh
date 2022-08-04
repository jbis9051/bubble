#!/usr/bin/env bash

set -e

DATABASE_URL=postgres://user:pass@localhost:5433/db

docker build . --tag db
docker run --rm --network="host" -e DATABASE_URL=$DATABASE_URL db migrate
docker run --rm --network="host" -e DATABASE_URL=$DATABASE_URL db revert
AMOUNT=$(docker run --rm --network="host" postgres psql $DATABASE_URL -AXqtc "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'public' AND table_name <> '_sqlx_migrations';")

if [ "$AMOUNT" -ne 0 ]; then
  echo "Database is not empty. This means your revert failed to actually revert the db. Tables left: $AMOUNT"
  echo "Tables: "
  docker run --rm --network="host" postgres psql $DATABASE_URL -c "SELECT * FROM information_schema.tables WHERE table_schema = 'public' AND table_name <> '_sqlx_migrations';"
  exit 1
fi