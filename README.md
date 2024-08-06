# mTLS Client Certificates with Axum (proxied)

## Setup

First, some TLS setup needs to be done:

```sh
cd certs/
./setup.sh                      # Generate required certificates
./frontend/generate-client.sh   # Create a client certificate with any username (repeatable)
```

Afterward, you can start the server:

```sh
cargo run
```

A reverse proxy should now handle mTLS and send the client certificate through the `X-Client-Cert` request header. Note that the proxy need to verify the client certificate and key as Axum will will not be able to in this configuration. [nginx.conf](nginx.conf) contains an example for Nginx.

### Docker

The setup described above can be replicated in a tested environment using the following command:

```sh
docker compose up --build
```

After it has started up, you should find the generated certificate files inside the mounted `certs/` directory.

## Connecting

For testing, `curl` can provide `--key` and `--cert` parameters, as well as `-k` to accept the self-signed certificate:

```sh
cd certs/frontend/
curl --key client-key_client1.pem --cert client-cert_client1.pem -k 'https://localhost/auth'
```

> Authenticated as "client1" (serial: 0x...)

In web browsers, import the `certs/client-cert_client1.pfx` file and restart the browser. This should show popup to select the certificate on visiting https://localhost. Check out https://localhost/auth to see if authentication was successful.
