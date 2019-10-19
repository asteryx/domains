
install:
	cargo install systemfd cargo-watch && cd src/ng && npm install
run:
	DATABASE_URL="data.sqlite" systemfd --no-pid -s http::8000 -- cargo watch -x run
ngbuild:
	cd src/ng/ && npm run watch