# Design
## Modules
- core
  - cryptography
    - key pair
      - generation
      - key to/from pem file
      - key to/from string (to be stored in db)
      - encrypt
      - decrypt
- server 
  - db entity & sql
  - api
- client 
- cli 
  - make use of client 
  - as the id as superuser
    - auth by the superuser's private key?

## Database
- Sqlite
- Tables
  - User
    - username (PK)
    - role - TEXT (app, admin, superuser)
    - path to the public key pem file
    - encrypted password
    - password salt
    - active - boolean, if the key is validated by handshake
  - Access_Right
    - admin's username (FK to user)
    - app name (FK to user)
      - that this admin has access right to
  - Config_data
    - app name (PK) (FK to app)
    - config key (PK)
    - value (encrypted, so in string format)
- all id in TEXT need to be 
  - in one word 
  - char in [a-Z, 0-9, _]

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
server direcory can be given by (distinguish by this order)
- param (--dir)
- global variable (linux) WELLIK_HOME
- current directory

```sh
# init the app
wellik init --file <path-to-config-file> --dir <optional path to the server dir>
# ask for input > root (superuser) password: 
# the app generate a root key pair 
# ask if wants to set the dir in the global var WELLIK_HOME (via figment)

# start/stop the server
wellik start --dir <server dir>   # start the server & write the pid to a file, need to be run with `nohup &`
wellik stop  --dir <server dir>   # kill the old server with the pidfile by process::Command

### the other commands required login
#### root at server machine:
wellik login root --dir <server dir>
# ask for input > password
# root user don't need to provide the private key as it is in the server base dir

#### other user at server / other machine:
wellik login <username> --server <server connection url> --key <private-key-file>
# ask for input > password
# the private key is for decrypted the config data

#############################################
### Commands in the app's shell after login #
#############################################
# create users of different role
create app <app-name> pubic-key-file
# ask for input > app user password: 

create admin <username> pubic-key-file --access app-name1,app-name2
# app: app that this admin has access to
# require root role
# ask for input > admin user password: 

## delete user
remove <username>

## update admin access right
alteradmin <username> add <app-name>
alteradmin <username> drop <app-name>

# get / set the config
get <app-name> <config-key>
set <app-name> <config-key> <config-value>

# subscribe the config-change channel
subscribe

### run a "script" that cointain the above commands
run <path-to-a-text-file>
```

## rust library
- web
  - https://github.com/tokio-rs/axum
  - auth: https://github.com/Owez/axum-auth
  - jwt: 
    - https://github.com/wpcodevo/rust-axum-jwt-auth
    - https://crates.io/crates/jwt
  - HTTPS: https://github.com/tokio-rs/axum/tree/main/examples/low-level-rustls
- web client
  - reqwest: https://github.com/seanmonstar/reqwest
- redis 
  - https://github.com/redis-rs/redis-rs?tab=readme-ov-file
- sqlite
  - SQL builder: https://github.com/SeaQL/sea-query
  - Executer / driver: https://github.com/launchbadge/sqlx
- Error
  - https://github.com/dtolnay/anyhow
  - https://github.com/dtolnay/thiserror
- clap - Command Line Argument Parser for Rust
  - https://docs.rs/clap/latest/clap/
  - example: https://github.com/tokio-rs/mini-redis/blob/master/src/bin/server.rs
- config parser
  - https://docs.rs/figment/latest/figment/
- rsa
  - https://docs.rs/rsa/latest/rsa/