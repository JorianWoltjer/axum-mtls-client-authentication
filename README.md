# mTLS Client Certificates with Axum

## Setup

First, some TLS setup needs to be done:

```sh
cd certs/
rm *.pem *.pfx *.csr *.srl
./generate-server.sh     # Server's self-signed TLS
./generate-client-ca.sh  # Create signing key for client certificates
./generate-client.sh     # Create client certificate with any username (repeatable)
```

Afterwards, you can start the server:

```sh
cargo run
```

### Docker

It seems like there is a problem with earlier openssl versions resulting in a `certificate unknown (558)` error. This can be resolved using a newer openssl version like 3.3.X. Alpine's [openssl](https://pkgs.alpinelinux.org/package/edge/main/armhf/openssl) package uses the latest version and is used in a Docker setup inside this repository. Use the following command to start it:

```sh
docker compose up --build
```

After it has started up, you should find the generated certificate files inside the mounted `certs/` directory.

## Connecting

For testing, `curl` can provide `--key` and `--cert` parameters, as well as `-k` to accept the self-signed certificate:

```sh
curl --key client-key.pem --cert client-cert.pem -k 'https://localhost:8443/auth'
```

In web browsers, import `certs/client-cert.pfx` file and restart the browser. This should show popup to select the certificate on visiting https://localhost:8443. Check out https://localhost:8443/auth to see if authentication was successful.
