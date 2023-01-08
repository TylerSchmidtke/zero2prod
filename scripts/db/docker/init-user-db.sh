#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
	CREATE USER newsletter WITH PASSWORD 'newsletter';
	CREATE DATABASE newsletter;
	GRANT ALL PRIVILEGES ON DATABASE newsletter TO newsletter;
EOSQL