FROM alpine:3.20.2

RUN apk add --no-cache bash openssl cargo
RUN adduser -D user

WORKDIR /app
COPY . .
RUN chown -R user:user /app
RUN chmod +x /app/certs/*.sh /app/entrypoint.sh
USER user
RUN cargo build --release

EXPOSE 8443

ENTRYPOINT [ "/app/entrypoint.sh" ]
CMD ["./target/release/mtls-client-authentication"]
