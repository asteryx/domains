run:
	systemfd --no-pid -s http::8000 -- cargo watch -x run
ngbuild:
	cd src/static/ && npm run watch