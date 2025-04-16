# Heterogenous Data Orchester

## Script parameters

### Set logging level

The logging level can be set via

```sh
-l <level>
```

with different the different level hierarchies [debug, error, warn, info, trace, off].

### Set config file

The startup config file can be set via

```sh
-f <path>
```

Where the starting path point is the root of the repository

## Config

### Generate

### Process

### Store

The available config parameters for a store nodes are given as 

```json
{
    
    "docker": {}
}
```

The configuration officially supports the databases [PostGres, Redis, MongoDB, Polypheny] through docker images.
