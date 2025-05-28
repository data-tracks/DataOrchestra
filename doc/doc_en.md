# Data Orchestra



## Script

The general script can be started through the normal cargo run command

```sh
cargo run -- <parameters>
```

### Set logging level

The logging level can be set via

```sh
-l | --level <level>
```

with different the different level hierarchies [`debug`, `error`, `warn`, `info`, `trace`, `off`].

### Set config file

The startup config file can be set via

```sh
-f | --file <path>
```

Where the starting path point is the root of the repository.

### Remove docker containers

All docker containers which exist can be automatically stopped and deleted with the additional flag
```sh
--remove_all
```

### Generate valid json

A sample json can be generated with the flag
```sh
--generate_valid_json
```

## Config

### Variables

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
Variables inside a `map` variable can also be accessed to any depth and can be accessed via a simple `.` scheme, `${<variable>.<variable>. ...}`.
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
### Object

An object is a generic component which has no specific capabilities. It acts as the skeleton of other more specialized components and can be used if no object meet ones specification for a component.

```json
{
  "start": "",
  "data": "",
  "node": {
    "name": "",
    "address": {
      "ip": "",
      "port": ""
    }
  },
  "docker": 
  {
    "name": "",
    "network": "",
    "options": { ... },
    "address": {
      "ip": "",
      "port": ""
    },
    "mount": ""
    "image": "",
    "compose": "",
    "file": ""
  },
  "attach": {} | []
}
```

### Generate

### Process

### Store

The available config parameters for a store nodes are given as 

```json
{
  /* All fields of object are inherited */
  "schema": " | []",
  "type": "",
  "config": {
    ...
  }
}
```

#### Type

Type of preconfigured database. The officially supported types are [`postgres`, `redis`, `mongodb`, `polypheny`] through docker images.

#### Config

