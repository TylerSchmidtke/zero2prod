POSTGRES_USER=postgres
POSTGRES_PASSWORD=password
DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@127.0.0.1:5432/newsletter

.PHONY: init-db
init-db:
	DATABASE_URL=${DATABASE_URL} sqlx migrate run