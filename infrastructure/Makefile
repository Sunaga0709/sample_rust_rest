validate-openapi:
	cd ../openapi && \
	openapi-generator validate -i openapi.yaml
gen-openapi:
	cd ../openapi && \
	openapi-generator generate -i openapi.yaml -g rust-server -o ../server
serve:
	cd ../server && cargo run
