#FROM alpine:3.19.1
FROM rust:1.67

WORKDIR /mnt/c/Source/hello-phext
COPY . .

RUN cargo install --path .
ENV ROCKET_ADDRESS=127.0.0.1
CMD ["hello-phext"]
#ENTRYPOINT /hello-phext
