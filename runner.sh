#!/bin/bash

DATABASE_URL="postgres://domains:defaultpassword@localhost:5432/domains"

case ${1} in
  "build")
  sudo apt install libpq-dev wkhtmltopdf apt-transport-https \
    ca-certificates curl gnupg-agent software-properties-common
	cargo install --force systemfd cargo-watch diesel_cli
	docker-compose --file docker-compose.yml up -d
 	make migration run
  ;;
  "migration")
  diesel ${1} ${2} ${3} --database-url=${DATABASE_URL} --migration-dir ./src/migrations
  ;;
  "print-schema")
  diesel --database-url=${DATABASE_URL} print-schema
  ;;
  "start")
  docker-compose --file docker-compose.yml up -d
  RUST_BACKTRACE=1 systemfd --no-pid -s http::8000 -- cargo watch -w src/ -d 0.3 -x run
  ;;
  "stop")
  docker-compose --file docker-compose.yml stop
  ;;
  "test")
  echo ${POSTGRES_USER:-domains}
  ;;

esac