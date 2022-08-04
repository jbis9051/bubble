#!/usr/bin/env bash

set -e

DATABASE_URL=postgres://user:pass@localhost:5433/db

docker build . --tag db
TABLES=$(docker run --rm --network="host" postgres psql $DATABASE_URL -AXqtc "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public' AND table_name <> '_sqlx_migrations';")

get_rows(){
  docker run --rm --network="host" postgres psql $DATABASE_URL -AXqtc "SELECT COUNT(*) FROM \"$1\";"
}

echo "$TABLES" | while read -r TABLE ; do
  ROWS=$(get_rows "$TABLE")
  if [ "$ROWS" -ne 0 ]; then
    echo "Table '$TABLE' is not empty. This means some cleanup failed to actually cleanup the db. Rows left: $ROWS"
    echo "Rows: "
    docker run --rm --network="host" postgres psql $DATABASE_URL -c "SELECT * FROM \"$TABLE\";"
    exit 1
  fi
done
