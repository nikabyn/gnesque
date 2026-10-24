#!/usr/bin/env bash

CHANGED=$(cargo build -r --bin gnesque-shell --message-format=json \
        | jq -s 'any(.[]; .reason == "compiler-artifact" and .fresh == false)')

if [[ "$CHANGED" == "true" ]]; then
    pkill -TERM -x gnesque-shell
    nohup cargo r -r --bin gnesque-shell </dev/null >/tmp/gnesque-shell.log 2>&1 &
    echo "Started PID $!"
fi
