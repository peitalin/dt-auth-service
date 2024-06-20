
#### use this file to test requests for logins
import requests
import json
url = "http://0.0.0.0:8082"

### 1. Query a user's public profile by id
### GET /user/get?user_id=user_id
res1 = requests.get(
    '{url}/user/get?user_id={user_id}'.format(
        url=url,
        user_id="1d0bf1a9-9578-49e0-a43b-ff7022ae35b6"
    )
)
json.loads(res1.content)


# 2. Try get profile without logging in: fail
### GET /auth/profile/get
res2 = requests.get('{url}/auth/profile/get'.format(url=url))
json.loads(res2.content)
### NOTE: no args required, email is extracted from JWT in session-cookie

### 3. Test Login
res3 = requests.post(
    '{url}/login'.format(url=url),
     json={"email": "severus@hogwarts.com", "password": ""}
)
json.loads(res3.content)
res3.cookies.get('dt-auth')

### 4. put cookies in request header and try get more data on prive profile
res4 = requests.get(
    '{url}/auth/profile/get'.format(url=url),
    cookies=res3.cookies
)
json.loads(res4.content)
### NOTE: no args required, email is extracted from JWT in session-cookie
### Profile retrieve is the email in the JWT


### 4b.
## JWT decoder route
res4b = requests.get(
    '{url}/auth/id'.format(url=url),
    cookies=res3.cookies
)
json.loads(res4b.content)

### 5. now test Logging out
res5 = requests.delete(
    '{url}/logout'.format(url=url),
    cookies=res3.cookies
)
json.loads(res5.content)

### 6. try get User profile again after logging out: fail
res6 = requests.get(
    '{url}/auth/profile/get'.format(url=url),
    json={"email": "severus@hogwarts.com" },
    cookies=res3.cookies
)
json.loads(res6.content)



### 7. Test user profile creation.
res7 = requests.post('{url}/user/create'.format(url=url),
    json={
        "email": "severus@hogwarts.com",
        "password": "",
        "username": "Halfblood Prince",
        "first_name": "Severus",
        "last_name": "Snape",
    })
json.loads(res7.content)



### Test email verification
res8 = requests.post('{url}/user/create'.format(url=url),
    json={
        "email": "severus@hogwarts.com",
        "password": "",
        "username": "severus",
        "first_name": "Severus",
        "last_name": "Snape",
    })
json.loads(res8.content)


### Test password reset email
res9 = requests.post(
    'http://0.0.0.0:8082/forgot/1/sendResetPasswordEmail',
    json={ "email": "severus@hogwarts.com" }
)
json.loads(res9.content)


res11 = requests.get(
    'http://0.0.0.0:8082/login',
    json={"email": "severus@hogwarts.com", "password": ""}
)
res11.content


## Updates profile using id
res14 = requests.post(
    '{url}/auth/profile/update'.format(url=url),
    json={
        "id": "093bd823-af01-4f86-9836-02260e989385",
        "username": "sirius"
    },
    cookies=res3.cookies
)
json.loads(res14.content)
