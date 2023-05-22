# API Routes

## User Signup

```http request
POST /user/register
```

#### Request:

```json
{
  "email": "<email>",
  "username": "<username>",
  "password": "<password>",
  "name": "<name>",
  "identity": "<identity>"
}
```

An email will be sent to the email address provided with a link to confirm the account.

#### Response:

```
201 Created
<uuid>
```

#### Error:

```
409 Conflict
<username|email>
```

---

## Email Confirm

```http request
POST /user/confirm
```

#### Request:

```json
{
  "token": "<token>"
}
```

#### Response:

```json
{
  "token": "<token>"
}
```

#### Error:

```
404 Not Found
```

**Note:** This will invalidate all session tokens for the user (excluding the session token sent in response).



---

## User Login

```http request
POST /user/session
```

#### Request:

```json
{
  "email": "<email>",
  "password": "<password>"
}
```

#### Response:

```json
{
  "token": "<token>"
}
```

## User Logout

```http request
DELETE /user/session
```

#### Request:

```json
{
  "token": "<token>"
}
```

#### Response:

```
200 OK
``` 

---

## User Forgot Password

```http request
POST /user/forgot
```

#### Request:

```json
{
  "email": "<email>"
}
```

#### Response:

```
200 OK
```

An email will be sent to the email address provided with a link to reset the password.

## User Forgot Password Check

```http request
GET /user/reset?token=<token>
```

```http request
200 OK
404 Not Found
```

## User Forgot Password Confirm

```http request
POST /user/reset
```

```json
{
  "password": "<password>",
  "token": "token"
}
```

---

## User Identity

The user identity is the long time public key used to identify and authenticate the User to other Users. Each Client for
this User will have their Client signing key signed by the User identity key to prove that the Client legitimately
belongs to the User.

Identity can be changed at any time, but the User will need to re-authenticate all Clients. This can be required if the
User loses their identity private key or simply to rotate the identity key. In the latter case, the User should sign the
new identity key with the old identity key to prove that the new identity key belongs to the User. They should then send
this signature to all groups that they are a member of so that the group can update their records.

`<identity>` and other byte strings are encoded as base64 strings.

### Update Identity

#### Request:

```http request
PUT /user/identity
```

```json
{
  "identity": "<identity>"
}
```

#### Response:

```
200 OK
```

## Get User

#### Request:

```http request
GET /user/<uuid>
```

#### Response:

```json
{
  "uuid": "<uuid>",
  "username": "<username>",
  "name": "<name>",
  "identity": "<identity>"
}
```

## Get User's Clients

#### Request:

```http request
GET /user/<uuid>/clients
```

#### Response:

```
200 OK
```

```json
{
  "clients": [
    {
      "user_uuid": "<user_uuid>",
      "uuid": "<uuid>",
      "signing_key": "<signing_key>",
      "signature": "<signature>"
    }
  ]
}
```

## Create Client

#### Request:

```http request
POST /client
```

```json
{
  "signing_key": "<signing_key>",
  "signature": "<signature>"
}
```

The `<signature>` must be a signature of the `<signing_key>` by the User's identity key.

#### Response

```text
201 Created
<uuid>
```

## Get Client

#### Request:

```http request
GET /client/:uuid
```

```
200 OK
```

```json
{
  "user_uuid": "<user_uuid>",
  "uuid": "<uuid>",
  "signing_key": "<signing_key>",
  "signature": "<signature>"
}
```

## Update Client

#### Request:

```http request
PATCH /client/:uuid
```

```json
{
  "signing_key": "<signing_key>",
  "signature": "<signature>"
}
```

#### Response

```
200 OK
```

## Delete Client

#### Request:

```http request
DELETE /client/:uuid
```

#### Response

```
200 OK
```

---

# KeyPackages

KeyPackages are one time use (with the exception of the last KeyPackage).

See the MLS spec for more information.



## Replace KeyPackages

Note: This will replace all existing KeyPackages for the Client.

**The credential within the KeyPackage must be equal to `keypackage_<user_uuid>_<client_uuid>`**

#### Request:

```http request
POST /client/<uuid>/key_packages
```

```json
{
  "key_packages": [
    {
      "key_package": "<key_package>"
    }
  ]
}
```

`<key_package>` is a base64 encoded byte string.

#### Response:

```
200 OK
```

## Get KeyPackage

This will retrieve a KeyPackage for the given Client (if one is available) and delete it from the database.

#### Request:

```http request
GET /client/<uuid>/key_package
```

#### Response:

```json
{
  "key_package": "<key_package>"
}
```

`<key_package>` is a base64 encoded byte string.

---

# Messages

Messages are information sent and received by clients, including location updates. Updates to Recipients are abstracted to the messages model layer, there are no direct routes. 

## Send Message

#### Request:

```http request
POST /message
```

```json
    {
      "client_uuids": "<client_uuids>",
      "message": "<message>"
    }
```

`<client_uuids` is a vector of strings and `<message>` is a base64 encoding of a string's bytes.

#### Response:

```
200 OK
```

#### Error:
```
400 Bad Request
404 Not Found
```

Error StatusCode::BadRequest is returned when the json fields are improperly formatted or missing. Error StatusCode::NotFound is returned when any of the client_uuids are not found in the database.

## Receive Message
#### Request
```http request
GET /message
```
```json
{
  "client_uuid": "<client_uuid>"
}
```
'<client_uuid>' is a String.

#### Response
```json
{
  "messages": "<messages>"
}
```
```http request
200 OK
```
`<messages>` is a vector of base64 encodings of a message's bytes.

#### Error
```http request
400 Bad Request
403 Forbidden
404 Not Found
```
StatusCode::BadRequest is returned when the json fields are improperly formatted or missing. StatusCode::Forbidden is returned when the client is not associated with the user.
StatusCode::NotFound is returned when the client_uuid or message is not found in the database.