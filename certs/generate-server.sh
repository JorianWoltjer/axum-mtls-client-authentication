#!/bin/bash
set -e

# Generate a self-signed server key and certificate

openssl ecparam -name prime256v1 -genkey -noout -out server-key.pem
openssl req -x509 -key server-key.pem -out server-cert.pem -sha256 -days 365 -subj "/CN=localhost"
