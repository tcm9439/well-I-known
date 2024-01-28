# Security Analysis
This note describe the process of accessing and modifying the configuration and proof that only the authorized user will be able to read / write the configuration values.

## Situation
- Client app A owns a config: $(K_1,V_1)$, where $K$ is the config key and $V$ is the config value that should be kept secrete.
- Only app A, App B, admin C and superuser D should be able to read the value $V_1$.
- Only app A, admin C and superuser D should be able to edit the value $V_1$.
- Server has the public keys of users A, B, C and D.

## Process
1. To set / update a configuration, user A sent $(K_1,V_1)$ to server via HTTPS POST.
2. Server check if user A has access right to update this configuration. If not, reject the request.
3. Server encrypt the value by user A, B, C and D's public keys to produce 4 encrypted value $E_A(V_1), E_B(V_1), E_C(V_1), E_D(V_1)$.
4. Server store 4 key-value records: $(K_1, E_A(V_1)),(K_1, E_B(V_1)),(K_1, E_C(V_1)),(K_1, E_D(V_1))$. Set / update process completed.
5. User B sent a get config command with param $K_1$ to server.
6. Server check if user B has access right to read this configuration. If not, reject the request.
7. Server get and sent the corresponding key-value records $(K_1, E_B(V_1))$ to user B.
8. User B decrypt the encrypted value with its private key to obtain the plaintext value $D_B(E_B(V_1))=V_1$.

## Analysis
### Step 1
- At the beginning, only User A knows the secret $V_1$.
- The request send to the server via HTTPS so the data $V_1$ in the request body will be encrypted. Only the server holding the private key of the HTTPS cert will be able to decrypt the body and get the value.

### Step 2 & 6
- Server prevent unauthorized access by RBAC.

### Step 3 & 4
- The server will never store the plaintext value $V_1$ in the database.
- Each stored encrypted value can only be decrypted by the corresponding user who hold the private key.
- The database (sqlite) is protected by file access right. 
- Storing users public keys in the database is safe as long as the user does not expose the private key.
- The one who has access to all database data will not be able to get the plaintext config values unless they also steal some user's private key. 

### Step 7
- The replay data is protected by HTTPS so only user B will be able to read the plaintext reply.

### Step 8
- Only user B can decrypted the value with his private key.
- If someone get the wrong record that does not belong to him, he will not able to decrypted the value with a mismatched key.

## Conclusion
The data is safe as long as the server and all users keep their private key secret.
