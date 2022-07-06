#! /bin/bash

java -jar bin/openapi-generator-cli.jar generate -i ./definition/api.yaml -g rust-server -o ./openapi/ --package-name openapi
