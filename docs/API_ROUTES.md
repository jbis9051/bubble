# API Routes


## Groups

### Creating a Group

```
POST /group/<name>
```

Response:

200 Success

```json
{
  "uuid": 1,
  "name": "<name>",
  "created": "<timestamp>"
}
```

### Get Information About Group

```
GET /group/<id>
```

Response:

```json
{
  "uuid": 1,
  "name": "<name>",
  "created": "<date>"
}
```

### Update Group Name

``` 
POST /group/<id>/name
```

Request:

```json
{
  "name": "<name>"
}
```

### Delete Group

```
DELETE /group/<id>
```

### Adding Users to Group

```
POST /group/<id>/new_users
```

Request Data:

```json
{
  "users": [
    1,
    2,
    4
  ]
}
```

### Remove Users From Group

```
POST /group/<id>/delete_users
```

Request Data:

```json
{
  "users": [
    1,
    2,
    4
  ]
}
```

### Users


### Create User, Signup

```
POST /user/signup
```

Request Data:

```json
{
  "username": "<username>",
  "password": "<password>",
  "email": "<email",
  "phone": "<phone>",
  "name": "<name>"

}

### Confirm Signup

```
POST /user/signup-confirm
```

### Signin

```
POST /user/signin
```

### Signout

```
POST /user/signout
```

### Forgot Password

```
POST /user/forgot
```

### Forgot Password Confirmation

```
POST /user/forgot-confirm
```

### Change Email

```
POST /user/change-email
```

### Delete User

```
DELETE /user/delete
