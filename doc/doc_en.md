# Data Orchestra

---

# Script

The general script can be started through the normal cargo run command

```sh
cargo run -- <parameters>
```

The possible parameters are displayed below

| short | long                    | env                   | Description                                             | Types                                                        | Default |
|-------|-------------------------|-----------------------|---------------------------------------------------------|--------------------------------------------------------------|---------|
| `-f`  | `--file`                | `FILE`                | Path of config file                                     | Path                                                         | `None`  |
| `-l`  | `--level`               | `LEVEL`               | Set the logging level                                   | `info`, `warn`, `error`, `debug` and `trace`                 | `info`  |
| -     | `--remove_all`          | `REMOVE_ALL`          | Remove all docker containers and networks before start  | bool                                                         | `false` |
| -     | `--generate_valid_json` | `GENERATE_VALID_JSON` | Get deserialized version of an object                   | `Store`, `Process`, `Generate`, `Object`, `Docker`, `Attach` | `None`  |                                  |
| -     | `--portainer`           | `PORTAINER`           | Create portainer managing system                        | bool                                                         | `true`  |                                  |
| `-s`  | `--ssh_key`             | `SSH_KEY`             | Path to private ssh key for ssh connection verification | Path                                                         | `None`  |                                  |
| `-a`  | `--api_only`            | `API_ONLY`            | Only create the API                                     | bool                                                         | `false` |                                  |
| `-i`  | `--isolate`             | `ISOLATE`             | Create object(s) in isolation from config               | Name(s) of objects in config                                 | `None`  |                                  |

# Config

This section explains how objects can be defined in a configuration file. The configuration file is a JSON file which contains

## Variables

The config file allows for variable setting. Variables can be set inside the config file inside the `variables` map. Non-recursive variables are allowed, i.e. variables which themselves do not contain any variables. Variables can be a simple `value`,`array` or `map`, where `map` variables can again contain variables of the same types.
```json
{
  "variables": {
    "LOCAL": {
      "name": "local-environment",
      "host": "0.0.0.0"
    }
  }
}
```
Variables can then be used inside the config file through the accessing pattern `${<variable>}`.
```json
{
  "object": {
    "node": "${LOCAL}"
  }
}
```
Which gets replaced to 
```json
{
  "object": {
    "node": {
      "name": "local-environment",
      "host": "0.0.0.0"
    }
  }
}
```
Variables inside a `map` variable can also be accessed to any depth and can be accessed via a simple `.` scheme, `${<variable>.<variable>. ...}`. Same goes for `array` types which can be indexed through `${<variable>.index}`.
```json
{
  "object": {
    "node": {
      "name": "localhost",
      "host": "${LOCAL.host}"
    }
  }
}
```
## Object

Object is a generic component which has no specific capabilities. It acts as the skeleton of other more specialized components and can be used if no object meet ones specification for a component.

### Basics

An object field in the JSON can consist of one or multiple instances. 

```json
{
  "object": { ... }
}
```
or
```json 
{
  "object": [
    { 
      ... 
    },
    { 
      ... 
    }
  ]
}
```

Basic fields are

| Field     | Type           | Required | Description                                 | Default                                      |
|-----------|----------------|----------|---------------------------------------------|----------------------------------------------|
| `name`    | `string`       | no       | Name of object                              | `object`                                     |
| `to`      | `list(string)` | no       | Which objects it send data to for the graph | Empty list                                   |
| `ignore`  | `bool`         | no       | If object should be ignored for the graph   | `false`                                      |
| `ansible` | `string`       | no       | Path of ansible setup script                | `services/scripts/ansible/ansible-setup.yml` |

#### Example

```json
{
  "object": {
    "name": "website",
    "ignore": "true"
  }
}
```

### Node

Every object has a node definition which defines where the containers and data gets uploaded to

| Field      | Type                  | Required | Description                | Default |
|------------|-----------------------|----------|----------------------------|---------|
| `name`     | `string`              | no       | Name of node               | -       |
| `host`     | `string (ip address)` | yes      | Host ip of node            | -       |
| `username` | `string`              | yes      | Username of ssh connection | -       |
| `password` | `string`              | no       | Password of ssh connection | -       |
| `ssh_port` | `int`                 | no       | Ssh port                   | 22      |

#### Example

```json
{
  "object": {
    "node": {
      "name": "node-76",
      "host": "100.30.154.76",
      "username": "root"
    }
  }
}
```

### Docker container

| Field                     | Type                                                  | Required | Description                               | Default                      |
|---------------------------|-------------------------------------------------------|----------|-------------------------------------------|------------------------------|
| `name`                    | `string`                                              | no       | Name of docker container                  | (docker default on creation) |
| `network`                 | `string`                                              | no       | Network of docker container               | orchestra                    |
| `environment`             | `map`                                                 | no       | Environment variables                     | -                            |
| `volumes`                 | `string \| list(string)`                              | no       | File / folder mounts for docker container | -                            |
| `compose`                 | `string`                                              | yes*     | Docker compose file                       | -                            |
| `dockerfile`              | `string`                                              | yes*     | Docker dockerfile                         | -                            |
| `image`                   | `string`                                              | yes*     | Docker image                              | -                            |
| `publish_all`             | `bool`                                                | no       | Publish all ports                         | false                        |
| `build_args`              | `map`                                                 | no       | Dockerfile building args                  | -                            |
| `interpolation_variables` | `map`                                                 | no       | Compose interpolation variables           | -                            |
| `restart`                 | `No \| OnFailure(integer) \| Always \| UnlessStopped` | no       | Restart policy for container              | `No`                         |

\* If a compose is given it doesn't require `dockerfile` and `image`. 
If a `dockerfile` is given, it doesn't require a `compose` file, but it does require an image.
If a `image` is given, it doesn't require a `compose` and `dockerfile`.

#### Example

```json
{
  "object": {
    "docker": {
      "name": "generator",
      "mount": [
        "./examples/smart_building/generator/energy_sensor:/energy_sensor"
      ],
      "dockerfile": "images/rust.dockerfile",
      "publish_all": true,
      "image": "rust_base"
    }
  }
}
```

### Resources 

Resources is a single `map` or a `list` of `map`'s of the following types. 

#### Data

Data is a resource which contains existing files from the current system and uploads them to the defined location.

| Field         | Type                | Required | Description                                                                                                                                    | Default     |
|---------------|---------------------|----------|------------------------------------------------------------------------------------------------------------------------------------------------|-------------|
| `location`    | `Node \| Container` | no       | Location of where the data is uploaded to. `Node` and `Container` is used in the context of the object the data is attached to                 | `Container` |
| `type`        | `data`              | yes*     | Type of resource                                                                                                                               | -           |
| `name`        | `string`            | no       | Name of container. Mainly for when multiple containers are defined under the same object. If there is only one container, this is not required | -           |
| `path`        | `string`            | yes      | Path of local directory / file                                                                                                                 | -           |
| `destination` | `string`            | yes      | Path of remote directory / file                                                                                                                | -           |
| `dependency`  | `string`            | no       | Path of script which is executed to download any required dependencies                                                                         | -           |

*`data` is required for the `map` to be recognised as a data resource

#### Volatile

Volatile is a resource which contains data which is written into a file on the defined location.

| Field         | Type                | Required | Description                                                                                                                                    | Default     |
|---------------|---------------------|----------|------------------------------------------------------------------------------------------------------------------------------------------------|-------------|
| `location`    | `Node \| Container` | no       | Location of where the volatile is uploaded to. `Node` and `Container` is used in the context of the object the volatile is attached to         | `Container` |
| `type`        | `volatile`          | yes*     | Type of resource                                                                                                                               | -           |
| `name`        | `string`            | no       | Name of container. Mainly for when multiple containers are defined under the same object. If there is only one container, this is not required | -           |
| `content`     | `string`            | yes      | Content to be written into file                                                                                                                | -           |
| `destination` | `string`            | yes      | Path of file                                                                                                                                   | -           |

*`volatile` is required for the `map` to be recognised as a volatile resource

#### Example

```json
{
  "object": {
    "resources": [
      {
        "type": "data",
        "location": "container",
        "path": "examples/smart_building/generator/energy_sensor",
        "destination": "/energy_sensor"
      },
      {
        "type": "volatile",
        "location": "container",
        "destination": "/energy_sensor/echo.sh",
        "content": "#!/usr/bin/bash\necho \"Energy Sensor\""
      }
    ]
  }
}
```

### Executables

Executables is a single `map` or a list of `map`'s. It signifies an executable process. 
This differs from [resources](#resources-), in the sense that it is assumed that the script is already at the `path` location. 
For uploading of the script to the remote location one should use [resources](#resources-). The following types exist

#### Script

| Field      | Type                | Required | Description                                                                                                                                    | Default     |
|------------|---------------------|----------|------------------------------------------------------------------------------------------------------------------------------------------------|-------------|
| `type`     | `script`            | yes*     | Type of executable                                                                                                                             |         |
| `location` | `Node \| Container` | no       | Location of where script is located and should be executed                                                                                     | `Container` |
| `name`     | `string`            | no       | Name of container. Mainly for when multiple containers are defined under the same object. If there is only one container, this is not required | -           |
| `path`     | `string`            | yes      | Path of script                                                                                                                                 | -           |

*`script` is required for the `map` to be recognised as a script executable

#### Tmux

| Field      | Type                | Required | Description                                                                                                                                    | Default     |
|------------|---------------------|----------|------------------------------------------------------------------------------------------------------------------------------------------------|-------------|
| `type`     | `tmux`              | yes*     | Type of executable                                                                                                                             |         |
| `location` | `Node \| Container` | no       | Location of where script is located and should be executed                                                                                     | `Container` |
| `name`     | `string`            | no       | Name of container. Mainly for when multiple containers are defined under the same object. If there is only one container, this is not required | -           |
| `path`     | `string`            | yes      | Path of script                                                                                                                                 | -           |
| `session`  | `string`            | yes      | Name of tmux session                                                                                                                           | -           |
| `commands` | `list(string)`      | yes      | Commands executed in tmux session. The commands are ordered                                                                                    | -           |

*`tmux` is required for the `map` to be recognised as a tmux executable

#### Example

```json
{
  "object": {
    "executables": [
      {
        "type": "script",
        "location": "container",
        "path": "/energy_sensor/start.sh"
      },
      {
        "type": "tmux",
        "location": "container"
      }
    ]
  }
}
```

### Attachables

Attachables are objects which can be attached to an already existing object serving a simple function. The following attachables exist

#### Kafka Producer

| Field      | Type                                           | Required | Description                                       | Default          |
|------------|------------------------------------------------|----------|---------------------------------------------------|------------------|
| `api_port` | `integer`                                      | no       | Port at which API can be accessed                 | 5000             |
| `address`  | `string`                                       | no       | Address of Kafka Stream Processing system         | `localhost:9092` |
| `level`    | ``info`, `warn`, `error`, `debug` and `trace`` | no       | Logging level                                     | `info`           |
| `topics`   | `list(string)`                                 | yes      | List of Kafka topics                              |                  |
| `logger`   | `string`                                       | no       | Address of Logging Kafka Stream Processing system | None             |

#### Kafka Consumer

| Field      | Type                                           | Required | Description                                       | Default          |
|------------|------------------------------------------------|----------|---------------------------------------------------|------------------|
| `address`  | `string`                                       | yes      | Address where data is send to                     | -                |
| `consumer` | `string`                                       | no       | Address of Kafka                                  | `localhost:9092` |
| `level`    | ``info`, `warn`, `error`, `debug` and `trace`` | no       | Logging level                                     | `info`           |
| `group_id` | `string`                                       | yes      | Group id of consumer                              | -                |
| `topics`   | `list(string)`                                 | yes      | List of Kafka topics                              | -                |
| `logger`   | `string`                                       | no       | Address of Logging Kafka Stream Processing system | None             |

#### Example

```json
{
  "attach": {
    "kafka_producer": {
      ""
    }
  }
}
```

## Generate

Generate is a specialization of an [object](#object). It represents an object which generates data. 
Additionally to the object configuration, it contains additional configuration options. 
The following subsections describe the configuration, where all can be placed into the configuration like this 

```json
{
  "generate": {
    "...": "..."
  } 
}
```

### Basics 

| Field    | Type                                           | Required | Description                             | Default |
|----------|------------------------------------------------|----------|-----------------------------------------|---------|
| `amount` | `integer`                                      | no       | Amount of the identical generate object | 1       |

### Types

A custom process type can be set the following way

```json
{
  "type": "NAME"
}
```

or set and customized

```json
{
  "NAME": {
    "...": "..."
  }
}
```

#### Sensor

Sensor can be customized through the following fields

| Field      | Type           | Required | Description                            | Default |
|------------|----------------|----------|----------------------------------------|---------|
| `interval` | `integer`      | no       | Interval in which data is send (in ms) | 1000    |
| `address`  | `string`       | yes      | Address of where data is send to       | -       |

It sends packages as a JSON to the given address

```json
{
  "id": "...",
  "value": ...,
  "timestamp": "..."
}
```

##### Example

```json
{
  "sensor": {
    "interval": 100,
    "address": "100.30.154.76"
  }
}
```

## Process

Process is a specialization of an [object](#object). It represents an object which processes data.
Additionally to the object configuration, it contains additional configuration options.
The following subsections describe the configuration, where all can be placed into the configuration like this

```json
{
  "process": {
    "...": "..."
  } 
}
```

### Types

A custom process type can be set the following way

```json
{
  "type": "NAME"
}
```

or set and customized

```json
{
  "NAME": {
    "...": "..."
  }
}
```

#### Kafka

Kafka can be customized through the following fields

| Field    | Type           | Required | Description                                  | Default     |
|----------|----------------|----------|----------------------------------------------|-------------|
| `topics` | `list(string)` | yes      | Topic names                                  | Empty       |
| `host` | `string `      | no       | Ip Address of the location of the Kafka host | `127.0.0.1` |

##### Example

```json
{
  "kafka": {
    "topics": ["input", "output"],
    "host": "100.30.154.76"
  }
}
```

## Store

Store is a specialization of an [object](#object). It represents an object which stores data.
Additionally to the object configuration, it contains additional configuration options.
The following subsections describe the configuration, where all can be placed into the configuration like this

```json
{
  "store": {
    "...": "..."
  } 
}
```

### Types

A custom store type can be set the following way

```json
{
  "type": "NAME"
}
```

or set and customized 

```json
{
  "NAME": {
    "...": "..."
  }
}
```

#### PostgreSQL

PostgreSQL can be customized through the following fields

| Field    | Type     | Required | Description                      | Default    |
|----------|----------|----------|----------------------------------|------------|
| `postgres_db` | `string` | no       | Database name                    | `postgres` |
| `postgres_user` | `string` | no       | Username                         | `postgres` |
| `postgres_password` | `string` | no       | User password                    | `postgres` |
| `postgres_initdb_args` | `string` | no       | Special initialization arguments | None       |

##### Example

```json
{
  "postgres": {
    "postgres_db": "database",
    "postgres_user": "user",
    "postgres_password": "password",
    "postgres_initdb_args": "--data-checksums"
  }
}
```

#### Redis

Redis currently has no customizable fields.

##### Example

```json
{
  "type": "redis"
}
```

#### MongoDB

MongoDB can be customized through the following fields


| Field    | Type      | Required | Description   | Default |
|----------|-----------|----------|---------------|---------|
| `username` | `string`  | no       | Username      | `mongo` |
| `password` | `string` | no       | User passworr | `mongo`        |

##### Example

```json
{
  "mongodb": {
    "username": "user",
    "password": "password"
  }
}
```

