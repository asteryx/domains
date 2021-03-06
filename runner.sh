#!/bin/bash

DATABASE_URL="postgres://domains:defaultpassword@localhost:5432/domains"

case ${1} in
  "build")
  sudo apt install -y libpq-dev wkhtmltopdf apt-transport-https \
    ca-certificates curl gnupg-agent software-properties-common \
    build-essential libsqlite3-dev libmysqlclient-dev
	cargo install --force systemfd cargo-watch diesel_cli
	docker-compose --file docker-compose.yml up -d
  ;;
  "migration")
  diesel ${1} ${2} ${3} --database-url=${DATABASE_URL} --migration-dir ./src/migrations
  ;;
  "print-schema")
  diesel --database-url=${DATABASE_URL} print-schema
  ;;
  "start")
  docker-compose --file docker-compose.yml up -d
  systemfd --no-pid -s http::8000 -- cargo watch -i "src/ng/**" -w src/ -d 0.3 -x run
  ;;
  "stop")
  docker-compose --file docker-compose.yml stop
  ;;
  "restart")
  docker-compose --file docker-compose.yml stop && docker-compose --file docker-compose.yml up -d
  ;;
  "down")
  docker-compose --file docker-compose.yml down
  ;;
  "logs")
  docker logs domains_frontend -f --tail 30
  ;;
  "test")
  echo ${POSTGRES_USER:-domains}
  ;;

esac
