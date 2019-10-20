
install:
	cargo install systemfd cargo-watch && cd src/ng && npm install
migration:
	./migration.sh $(MAKECMDGOALS)
start:
	DATABASE_URL="db.sqlite" systemfd --no-pid -s http::8000 -- cargo watch -x run
ngbuild:
	cd src/ng/ && npm run watch
print-schema:
	diesel --database-url db.sqlite print-schema
%:
	@: