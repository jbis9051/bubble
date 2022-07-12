stop: stop_ci

stop_ci:
	docker compose -f docker-compose-ci.yml down

test_db: stop_ci
	docker compose -f docker-compose-ci.yml up --build -d
	cd db && ./test.sh && cd ..

fmt:
	cargo fmt --all
	cargo clippy --fix --allow-dirty --allow-staged

dev:
	docker compose -f docker-compose-dev.yml down
	docker compose -f docker-compose-dev.yml up --build