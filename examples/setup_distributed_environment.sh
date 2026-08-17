#!/bin/bash

cargo run --bin data_orchestra_master -- --config-file ./examples/config.toml &

sleep 10

cargo run --bin data_orchestra_master -- --config-file ./examples/test/test-config.toml &
