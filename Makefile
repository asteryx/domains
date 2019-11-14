
install:
	sudo apt install libmysqlclient-dev libsqlite3-dev libpq-dev && cargo install systemfd cargo-watch diesel_cli && cd src/ng && npm install
migration:
	./migration.sh $(MAKECMDGOALS)
start:
	DATABASE_URL="db.sqlite" RUST_BACKTRACE=1 systemfd --no-pid -s http::8000 -- cargo watch -x run
ngbuild:
	cd src/ng/ && npm run watch
print-schema:
	diesel --database-url db.sqlite print-schema
%:
	@:
