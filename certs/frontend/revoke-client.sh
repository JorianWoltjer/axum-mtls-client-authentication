#!/bin/bash
cd "$(dirname "$0")"  # Change to the directory of the script
. ../lib.sh
# === Revoke a client certificate by adding it to the CRL ===
# === Usage: ./revoke-client.sh <client-cert.pem> ===

if [ ! -f ca-cert.pem ]; then
  echo "Error: ca-cert.pem not found. Run setup.sh first."
  exit 1
fi

if [ "$#" -ne 1 ]; then
  echo "Usage: $0 <client-cert.pem>"
  exit 1
fi

CERT="$1"

revoke_client_cert "$CERT"
reload_nginx
