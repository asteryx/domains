#!/bin/bash
#echo ${2}
diesel ${1} ${2} ${3} --database-url db.sqlite --migration-dir ./src/migrations