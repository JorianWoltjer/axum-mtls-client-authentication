#!/bin/bash
cd "$(dirname "$0")"  # Change to the directory of the script
. ../lib.sh
# === Generate a client certificate and sign with CA ===
# === Usage: ./generate-client.sh [name] ===

USERNAME=${1:-$(whoami)}

if [ ! -f ca-cert.pem ]; then
  echo "Error: ca-cert.pem not found. Run setup.sh first."
  exit 1
fi

generate_client_cert "$USERNAME" true
