# WellIKnown - A Config Manager

> A simple project for me to learn Rust :)

This project aim to provide centralized server to store and manage configuration of different application and services. Simply speaking, it is a secured key-value store with read / write via Restful API and access control.

## Functionality
### Key-Value store
- The config are managed in a key-value manner. 
  - Key => string
  - Value => as encrypted, also string
    - The output-size should always equals the size of the Modulus (part of the key)
    - Ref: https://stackoverflow.com/questions/25699187/rsa-encryption-output-size
- Each app has many config keys

### Access Control & Security
- The API and requests are protected by HTTPS.
- Each data is encrypted by at least two (pair of) key to produce two ciphers text using RSA
- One key is the app-level key that is known only by the app that would use the data
- Other key is hold by the other user having the access right (e.g. admin). 
- RBAC Role: 
  - app (operator, developer for one special app)
    - only has access to the data for that app
  - admin
    - has access to the data for 1+ app
  - superuser / root
    - has access to all data for the app
- All user / role has full read / write right as long as it has access to the data

### Value change notification 
- Using Redis pub sub to notify the client when there is update in any config.
- When the client receive the push, they may for example reload the application to apply the new config.
- Msg:
  - type: add / update / delete
  - app-name
  - config-key

### Server API
- The manager host a http server for the user (app / admin) to get, add, delete, update config
