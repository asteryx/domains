#!/bin/bash

case ${1} in
  "frontend")
  nodejs -v
  node_status=$?

  npm -v
  npm_status=$?

  if [ $node_status -ne 0 ] || [ $npm_status -ne 0 ]
  then
    apt-get update
    sudo apt-get install -y curl python-software-properties
    curl -sL https://deb.nodesource.com/setup_13.x | sudo bash -
    apt-get install -y nodejs npm
  fi
 	cd /app && npm install
 	npm run watch
  ;;

esac
