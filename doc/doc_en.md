# Heterogenous Data Orchester



## Script

The general script can be started through the normal cargo run command

```sh
cargo run -- <parameters>
```

### Set logging level

The logging level can be set via

```sh
-l <level>
```

with different the different level hierarchies [`debug`, `error`, `warn`, `info`, `trace`, `off`].

### Set config file

The startup config file can be set via

```sh
-f <path>
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

