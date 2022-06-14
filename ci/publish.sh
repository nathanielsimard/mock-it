#!/bin/bash

crate=$1

case ${crate} in
    mock-it_codegen)
        cd mock-it_codegen/
        echo "Publishing mock-it_codegen ..."
        ;;
    mock-it)
        echo "Publishing mock-it ..."
        ;;
    *)
        echo "Crate ${crate} unknown"
        exit 1
        ;;
esac

cargo publish --token ${CRATES_IO_API_TOKEN} || exit 1
echo "Sucessfully published ${crate}"
