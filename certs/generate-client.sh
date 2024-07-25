#!/bin/bash
set -e

# Generate a client certificate and sign with CA
USERNAME=${1:-$(whoami)}

if [ ! -f ca-key.pem ]; then
  echo "Error: ca-key.pem not found. Run generate-client-ca.sh first."
  exit 1
fi

openssl req -newkey rsa:4096 -nodes -keyout client-key.pem -out client-cert.csr -subj "/CN=$USERNAME"
openssl x509 -req -in client-cert.csr -CA ca-cert.pem -CAkey ca-key.pem -CAcreateserial -out client-cert.pem -days 365
openssl pkcs12 -export -keypbe NONE -certpbe NONE -nomaciter -passout pass: -inkey client-key.pem -in client-cert.pem -out client.pfx
