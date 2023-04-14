# API Routes

## User Signup

```http request
POST /user/register
```

#### Request:

```json
{
  "email": "<email",
  "username": "<username>",
  "password": "<password>",
  "name": "<name>"
}
```

An email will be sent to the email address provided with a link to confirm the account.

#### Response: 
```
201 Created
```

#### Error:

```
409 Conflict
<usernae|email>
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

---
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

---


# User Forgot Password Check

```http request
GET /user/reset
```
```http request
200 OK
404 Not Found
```

# User Forgot Password Confirm

```http request
POST /user/reset
```
```json
{
  "password": "<password>",
  "token": "token"
}
```


# Ignore Everything Below
## User Forgot Password Confirm

POST /user/change-email
```

### Delete User

```
DELETE /user/delete
