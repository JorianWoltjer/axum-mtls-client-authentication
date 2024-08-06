FROM alpine:3.20.2

RUN apk add --no-cache bash openssl cargo
RUN adduser -D user

WORKDIR /app

COPY src /app/src
COPY Cargo.* /app
RUN chown -R user:user /app
RUN cargo build --release

COPY certs /app/certs
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /app/certs/*.sh /entrypoint.sh

USER user
EXPOSE 8443

ENTRYPOINT [ "/entrypoint.sh" ]
CMD ["./target/release/mtls-client-authentication"]
