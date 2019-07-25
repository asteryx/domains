
install:
	cargo install systemfd cargo-watch && cd src/ng && npm install
run:
	systemfd --no-pid -s http::8000 -- cargo watch -x run
ngbuild:
	cd src/ng/ && npm run watch