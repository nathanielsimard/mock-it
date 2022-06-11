#!/bin/bash

echo "Publishing mock-it_codegen ..."
cd mock-it_codegen/
cargo publish --token ${CRATES_IO_API_TOKEN}
echo "Sucessfully published mock-it_codegen"

echo "Waiting 10 seconds for mock-it_codegen to be available ..."
sleep 10

echo "Publishing mock-it ..."
cd ../
cargo publish --token ${CRATES_IO_API_TOKEN}
echo "Sucessfully published mock-it"
