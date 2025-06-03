# Data-Orchestra

A rust based dynamic heterogeneous distributed data landscape generator.

## Quickstart

To test the system run the following command which runs the smart building testing example with the relevant variables. 
Note that you may need to change the nodes addresses to fit your needs.

```shell
cargo run -- -f examples/smart_building/config.json -s <path/to/your/ssh_key>
```

## Dependencies

- [log](https://github.com/rust-lang/log)
- [env_logger](https://github.com/rust-cli/env_logger)
- [clap](https://github.com/seanmonstar/reqwest)
- [reqwest](https://github.com/seanmonstar/reqwest)
- [tokio](https://github.com/tokio-rs/tokio)
- [serde](https://github.com/serde-rs/serde)
- [serde_json](https://github.com/serde-rs/json)
- [ssh2](https://github.com/alexcrichton/ssh2-rs)
- [ansi_term](https://github.com/ogham/rust-ansi-term)
- [walkdir](https://github.com/BurntSushi/walkdir)
- [regex](https://github.com/rust-lang/regex)
- [rstest](https://github.com/la10736/rstest)



