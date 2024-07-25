#!/bin/bash
set -e

# Generate a self-signed server key and certificate

openssl req -x509 -newkey rsa:4096 -nodes -keyout server-key.pem -out server-cert.pem -sha256 -days 365 -subj "/CN=localhost"
