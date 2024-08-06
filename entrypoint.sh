#!/bin/bash
set -e

cd /app/certs
if [ ! -f frontend/ca-cert.pem ] || [ ! -f backend/ca-cert.pem ]; then
  echo "Generating certificates..."
  ./setup.sh
  ./frontend/generate-client.sh client1
else
  echo "Certificates already exist."
fi

cd /app
exec "$@"
