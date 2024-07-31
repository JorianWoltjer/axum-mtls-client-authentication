#!/bin/bash
set -e

# Generate a client certificate CA signing key
CA_NAME=${1:-"Client CA"}

openssl ecparam -name prime256v1 -genkey -noout -out ca-key.pem
openssl req -x509 -key ca-key.pem -out ca-cert.pem -days 365 -subj "/CN=$CA_NAME"
