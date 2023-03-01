validate-openapi:
	cd ../openapi && \
	openapi-generator validate -i openapi.yaml
gen-openapi:
	cd ../openapi && \
	openapi-generator generate -i openapi.yaml -g rust-server -o ../server
serve:
	docker-compose exec app cargo run
up:
	docker-compose up -d
down:
	docker-compose down
build:
	docker-compose build
restart:
	docker-compose down && docker-compose up -d