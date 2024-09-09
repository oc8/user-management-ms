#!/bin/bash

FILE_PATH="./libs/gen/src/auth.rs"
IMPORTS="use apistos::ApiComponent;\\nuse schemars::JsonSchema;\\n"

# Example content
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    sed -i "1s/^/$IMPORTS/" "$FILE_PATH"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    sed -i '' "1s/^/$IMPORTS/" "$FILE_PATH"
else
    echo "Unsupported OS"
    exit 1
fi
