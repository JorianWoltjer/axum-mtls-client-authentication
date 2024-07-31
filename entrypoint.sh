#!/bin/bash
set -e

cd /app/certs
if [[ ! -f ca-cert.pem || ! -f server-key.pem || ! -f server-cert.pem ]]; then
  echo "Generating certificates..."
  ./generate-server.sh
  ./generate-client-ca.sh
  ./generate-client.sh client1
else
  echo "Certificates already exist."
fi

cd /app
exec "$@"
