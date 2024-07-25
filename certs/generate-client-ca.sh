#!/bin/bash
set -e

# Generate a client certificate CA signing key
CA_NAME=${1:-"Client CA"}

openssl req -x509 -newkey rsa:4096 -nodes -keyout ca-key.pem -out ca-cert.pem -days 365 -subj "/CN=$CA_NAME"
