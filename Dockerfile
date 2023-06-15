# See https://hub.docker.com/_/rust/

FROM rust:alpine as builder
WORKDIR /usr/src/vale2junit
COPY . .
RUN apk update
RUN apk add musl-dev
RUN CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse cargo install --path .

FROM alpine:latest
COPY --from=builder /usr/local/cargo/bin/vale2junit /usr/local/bin/vale2junit
RUN apk update
RUN apk add curl
# When running this container interactively, use `-v .:/mnt/vale2junit:Z`
# to mount the current directory in the host to the container working dir.
VOLUME ["/mnt/vale2junit"]
WORKDIR "/mnt/vale2junit"
CMD ["vale2junit"]
