stop: stop_ci

stop_ci:
	docker compose -f docker-compose-ci.yml down

test_db: stop_ci
	docker compose -f docker-compose-ci.yml up -d
	cd db && ./test.sh && cd ..