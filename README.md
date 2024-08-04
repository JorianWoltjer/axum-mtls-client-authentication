# mTLS Client Certificates with Axum (proxied)

## Setup

First, some TLS setup needs to be done:

```sh
cd certs/
rm *.pem *.pfx *.csr *.srl
./generate-server.sh     # Server's self-signed TLS
./generate-client-ca.sh  # Create signing key for client certificates
./generate-client.sh     # Create client certificate with any username (repeatable)
```

Afterward, you can start the server:

```sh
cargo run
```

A reverse proxy should now handle TLS and send the client certificate through the `X-Client-Cert` request header. [default.conf](default.conf) contains an example for Nginx.

### Docker

The setup described above can be replicated in a tested environment using the following command:

```sh
docker compose up --build
```

After it has started up, you should find the generated certificate files inside the mounted `certs/` directory.

## Connecting

For testing, `curl` can provide `--key` and `--cert` parameters, as well as `-k` to accept the self-signed certificate:

```sh
curl --key client-key.pem --cert client-cert.pem -k 'https://localhost/auth'
```

In web browsers, import the `certs/client-cert.pfx` file and restart the browser. This should show popup to select the certificate on visiting https://localhost. Check out https://localhost/auth to see if authentication was successful.
