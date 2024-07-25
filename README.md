# mTLS Client Certificates with Axum

## Certs Setup

1. generate server
2. generate CA
3. generate client

```sh
openssl pkcs8 -nocrypt -inform PEM -outform DER -in server-key.pem -out server-key.der
openssl pkcs12 -inkey client-key.pem -in client-cert.pem -export -out client.pfx
```

## Connecting

```
curl --cert client-cert.pem --key client-key.pem -k 'https://localhost:8443/'
```

For Chrome, import `.pfx` file and restart browser. This should show popup to select cert
