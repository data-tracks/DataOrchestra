[![Build Status](https://github.com/data-tracks/DataOrchestra/actions/workflows/rust.yml/badge.svg)](https://github.com/data-tracks/DataOrchestra/actions)
![Version Badge](https://img.shields.io/badge/Version-rust%201.88-orange?style=flat)
[![License Info](http://img.shields.io/badge/license-Apache%202.0%20License-brightgreen.svg)](https://github.com/data-tracks/DataOrchestra/blob/main/LICENSE)


# Data-Orchestra

A rust based dynamic heterogeneous distributed data landscape generator. Using a config and custom-made components one can easily deploy and test distributed landscapes.

## Quickstart

Start the local docker daemon, then run the following command which runs the smart building testing example with the relevant variables. 
Note that you may need to change the nodes addresses to fit your needs.

```shell
cargo run -- -f examples/smart_building/config.json -s <path/to/your/ssh_key>
```

## Features

The orchestrator provides multiple features which simplify the observing and interaction with the distributed landscape.

### Portainer

[Portainer](https://www.portainer.io/) is automatically integrated in the distributed landscape. 
At every remote node a portainer agent is deployed which allows for the interaction with the docker containers at that location through the portainer provide website at the url [https://localhost:9443](https://localhost:9443).
The default login for portainer is `admin` with `portaineradmin`.

### API

The Orchestrator deploys a local API which houses basic functionality such as message logging.

### Website

A website is provided which intergrates the communication with the API and portainer. See [https://github.com/data-tracks/TrackOrchestraView](https://github.com/data-tracks/TrackOrchestraView).

## Documentation

For the documentation on run parameters and a configuration guide, please check the [documentation](./doc/doc_en.md).

## Security warnings

This program is in no way tested for security vulnerabilities. 
This program contains a lot of user made remote code execution through script uploading and execution and should therefor be handled with care.
Additionally, the program requires the path to your private ssh key. While it only uses this for the establishing of the ssh connection, use at your own risk.

## Requirements

- Docker desktop
- rsync CLI (optional)

## License 

[Apache-2.0 license](./LICENSE)


