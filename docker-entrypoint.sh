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
    apt-get install -y curl
    curl -sL https://deb.nodesource.com/setup_13.x | bash -
    apt-get install -y nodejs
    cd /app && npm install
  fi
  echo fs.inotify.max_user_watches=524288 | tee -a /etc/sysctl.conf sysctl -p
 	cd /app && npm update
 	npm run watch
  ;;

esac
