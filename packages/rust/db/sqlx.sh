#!/usr/bin/env bash

set -e

if [ "$1" == "migrate" ]; then
  echo "Migrating database"
  sqlx migrate run --source /migrations --database-url "${DATABASE_URL}"
  exit 0
elif [ "$1" == "revert" ]; then
  echo "Reverting database"
  sqlx migrate revert --source /migrations --database-url "${DATABASE_URL}"
  exit 0
else
  echo "Usage: $0 [migrate|revert]"
  exit 1
fi
