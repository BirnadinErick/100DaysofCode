#!/bin/bash

LICENSE_FILE="license.txt"
if [ ! -f "$LICENSE_FILE" ]; then
    echo "License file not found!"
    exit 1
fi

find . -name '*.rs' -exec sh -c 'cat "$1" "$0" > /tmp/out && mv /tmp/out "$0"' {} "$LICENSE_FILE" \;

echo "License headers added to all Rust files."

