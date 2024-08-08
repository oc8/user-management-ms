#!/bin/bash

FILE_PATH="./libs/gen/src/auth.rs"

IMPORTS="use apistos::ApiComponent;\nuse schemars::JsonSchema;\n"

sed -i "1i $IMPORTS" "$FILE_PATH"
