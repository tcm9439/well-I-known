# Design
## Modules
- core
  - common obj - like the API para object
- server 
  - db stuff
- client 
- cli - make use of client

## Database
- Sqlite
- Tables
  - User
    - name (PK)
    - role
    - pub key
    - encrypted password salt
    - encrypted password
  - Admin_Access_Right
    - admin's user id (FK to user)
    - app that this admin has access right to
  - Application
    - name (PK) (FK to user)
    - brief description (nullable)
  - Config_data
    - app name (PK) (FK to app)
    - key (PK)
    - value (encrypted, so in string format)

## Server files
- superuser private & public key
  - owner: superuser, i.e. user that init & run the app 
  - access right: rwx --- ---
  - generate when the app first init, by admin cli
- HTTPS cert
  - owner: superuser
  - access right: rwx --- ---
- sqlite file
  - owner: superuser, i.e. user that init the app 
  - access right: rwx --- ---
- cli tool
- server config

## Server Config
- redis server connection link, auth info etc
- redis channel name
- cert filepath for HTTPS

## CLI
```sh
# init the app
cli-name init --file config-file
# ask for input > root (superuser) password: 
# the app generate a root key pair 

cli-name create app --name cli-name --pk pubic-key-file --info description
# ask for input > app user password: 

cli-name create admin --name username --pk pubic-key-file --app cli-name1,cli-name2
# ask for input > admin user password: 

cli-name drop app
cli-name drop admin

cli-name alter admin add cli-name
cli-name alter admin drop cli-name

# get / set the config
cli-name get cli-name config-key
cli-name set cli-name config-key

# subscribe the config-change channel
cli-name subscribe
```

## rust
- web
  - https://github.com/tokio-rs/axum
  - HTTPS: https://github.com/tokio-rs/axum/tree/main/examples/low-level-rustls
- redis 
  - https://github.com/redis-rs/redis-rs?tab=readme-ov-file
- sqlite
  - https://github.com/diesel-rs/diesel
- clap - Command Line Argument Parser for Rust
  - https://docs.rs/clap/latest/clap/
  - example: https://github.com/tokio-rs/mini-redis/blob/master/src/bin/server.rs
- ini config parser
  - https://docs.rs/configparser/latest/configparser/
- rsa
  - https://docs.rs/rsa/latest/rsa/