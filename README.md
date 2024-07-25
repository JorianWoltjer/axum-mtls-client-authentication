# mTLS Client Certificates with Axum

## Setup

First, some TLS setup needs to be done:

```sh
cd certs/
./generate-server.sh     # Server's self-signed TLS
./generate-client-ca.sh  # Create signing key for client certificates
./generate-client.sh     # Create client certificate with any username (repeatable)
```

Afterwards, you can start the server:

```sh
cargo run
```

## Connecting

For testing, `curl` can provide `--key` and `--cert` parameters, as well as `-k` to accept the self-signed certificate:

```sh
curl --key client-key.pem --cert client-cert.pem -k 'https://localhost:8443/auth'
```

In web browsers, import `certs/client-cert.pfx` file and restart the browser. This should show popup to select the certificate on visiting https://localhost:8443. Check out https://localhost:8443/auth to see if authentication was successful.
