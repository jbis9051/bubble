fmt:
	cargo fmt --all
	cargo clippy --fix --allow-dirty --allow-staged

dev:
	docker compose -f docker-compose-dev.yml down
	docker compose -f docker-compose-dev.yml up --build

test_ci: fmt_ci test_db_ci test_server_ci stop_ci

# Below is used for testing the CI environment. You can run the tests yourself if you would like with make test_ci
stop_ci:
	docker compose -f docker-compose-ci.yml down

start_ci:
	docker compose -f docker-compose-ci.yml up --build -d

test_db_ci: stop_ci start_ci
	cd db && ./revert_test.sh && cd ..

test_server_ci: stop_ci start_ci
	docker build db --tag db
	docker run --rm --network="host" -e DATABASE_URL=postgres://user:pass@localhost:5433/db db migrate
	cd server && DB_URL=postgres://user:pass@localhost:5433/db cargo test --tests && cd ..
	cd db && ./cleanup_test.sh && cd ..

fmt_ci:
	cargo fmt --all -- --check
	cargo clippy --all