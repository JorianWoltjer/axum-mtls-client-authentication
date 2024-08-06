#!/bin/bash
cd "$(dirname "$0")"  # Change to the directory of the script
. lib.sh
# === Perform the initial SSL setup ===

# Check if already setup and prompt to continue
if [ -f frontend/ca-cert.pem ] && [ -f backend/ca-cert.pem ]; then
  read -p "Certificates already exist. Do you want to continue and overwrite them? [y/N] "
  if [[ ! $REPLY =~ ^[Yy] ]]; then
    exit 1
  fi
fi

# Clean up any existing files and setup new ones
cd frontend/
cleanup
setup

# Generate a self-signed server key and certificate for HTTPS
generate_server_cert "localhost"
# Generate a client certificate CA signing key and CRL
generate_ca_cert "Client CA"
generate_crl
cd ..

# Clean up any existing files
cd backend/
cleanup

# Generate a backend server key and certificate for HTTPS
generate_server_cert "backend"
# Generate a backend client certificate CA signing key
generate_ca_cert "Backend CA"
# Generate a backend client certificate and sign with CA
generate_client_cert "nginx"

echo "Setup finished. Next, try generating a client certificate in frontend/ with ./generate-client.sh"
