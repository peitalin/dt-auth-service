
# User Service

This repository currently contains:
* User services: Signup, login, password resets,

<a name="table-of-contents"></a>
## Table of contents

<!--ts-->
   * [Table of contents](#table-of-contents)
   * [Basic Installation](#basic-installation)
   * [Docker Installation](#docker-installation)
      * [Docker Build Instructions](#building-docker)
      * [Misc. Docker Commands](#docker-misc-commands)
<!--te-->

<a name="basic-installation"></a>
## Basic Installation

Quickstart: Install Rust from [www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install):
```
curl https://sh.rustup.rs -sSf | sh
export PATH="$HOME/.cargo/bin:$PATH"
```
This will also install the rust package manager `cargo`. Then run:
```bash
git clone https://github.com/peitalin/dt-user-service
cd ./dt-user-service
cargo build
```

Then you can run the user service locally:
```bash
cargo test
cargo run --bin user
```

Test scripts to run interactively in ipython and node: `./scripts/client_login.py` are available.


* [Back to Table of Contents](#table-of-contents)
---

<a name="docker-installation"></a>
## Docker
You will need to install and run docker. Install Docker:
```bash
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh
```

<a name="building-docker"></a>

* **PS**: You need to set `-e JWT_DOMAIN="127.0.0.1"` for local development to get 'set-cookies' credentials to work.
* HttpOnly cookies will not be set if there is domain mismatch, it's automatically set.
* **PPS**: You also cannot manually access `HttpOnly` cookies, even if you can see them in Chrome dev tools (application tab).
* The cookie-credential is named "dt-auth", and is a hash of a JWT.
* This cookie-credential is forwarded with every request from a client. The server decodes and checks the JWT to determine auth-access rights.
* Logouts (JWT revokes) will not work without redis (which is included via docker-compose, but not in this image).
* Set GraphIql url explicitly. Nginx/docker changes hosts.

Then push to Google Container Registry
```bash
docker push gcr.io/dt-production/dt-user-service:latest
```


<a name="docker-misc-commands"></a>
### Misc. Docker Commands
To remove old containers and images, try:
```bash
### Remove images
docker rmi <image-id>
docker rm <container-id>
docker image prune

### Stop and remove all containers
docker stop $(docker ps -a -q)
docker rm $(docker ps -a -q)
`


