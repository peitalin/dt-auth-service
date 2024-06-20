# syntax=docker/dockerfile:experimental

############################################################
# NOTE: executables are in /target/release, instead of /target/debug
# For more info on multi-stage builds see:
# https://blog.jawg.io/docker-multi-stage-build/
############################################################

FROM rust:1.56-buster as builder
# Set working directory in Docker Image
WORKDIR /

# Copy cargo: contains list of dependencies
# Doing this first downloads and caches the libraries
COPY ./Cargo.toml    /Cargo.toml
# Copy over source files incrementally, to take advantage of cacheing
COPY ./src/db        /src/db
COPY ./src/utils     /src/utils
COPY ./src/lib.rs    /src/lib.rs
# Build lib: This also gets the dependencies cached
# RUN --mount=type=cache,target=/usr/local/cargo,from=rust,source=/usr/local/cargo \
#     --mount=type=cache,target=target \
#     cargo build --lib
RUN cargo build --lib

######### Build #########
# Copy over app files for each binary
COPY ./src/bin/user   /src/bin/user
RUN cargo build --bin user
# Build app (executables will be in /target/debug/)

### You can strip the binary to make it much smaller,
### but it also removes debug information.
RUN strip /target/debug/user

# print and inspect compiled binaries
RUN  ls /target/debug/
RUN  ls /

#######################################
####### END OF RUST BUILD STAGE #######
#######################################

### NOTE: the images must match the image used in the build stage
#### Debian
# Use stretch-slim debian, match image used in build stage.
FROM debian:buster-slim
RUN apt-get update && apt-get install -y libpq-dev curl

# Copy binaries from builder to this new image
COPY --from=builder /target/debug/user   /bin/dt_user

####### MUST INCLUDE FOR TLS requests to verify
COPY --from=builder /etc   /etc

#### Must run a long-running event to keep container alive
### Otherwise Kubernetes CrashLoopBackOff errors occur.
# CMD [ "tail", "-f", "/dev/null" ]
CMD ["dt_user"]
# Run commands in docker-compose.yml for each executable

# Indicate that this image expects to accept traffic internally on this port.
# NOTE: Expose doesn't do anything, it's just documenting that this port is hardcoded internally
# and you'll want to map a host port to this value.
EXPOSE 8082

# Define the health check
HEALTHCHECK --start-period=30s CMD curl --fail http://localhost:8082/_health || exit 1

