# See https://hub.docker.com/_/rust/
FROM rust:latest as builder
WORKDIR /usr/src/vale2junit
COPY . .
RUN cargo install --path .

FROM registry.access.redhat.com/ubi9-minimal:latest
COPY --from=builder /usr/local/cargo/bin/vale2junit /usr/local/bin/vale2junit
# When running this container interactively, use `-v .:/mnt/vale2junit:Z`
# to mount the current directory in the host to the container working dir.
VOLUME ["/mnt/vale2junit"]
WORKDIR "/mnt/vale2junit"
CMD ["vale2junit"]
