#!/bin/bash
# Import using `. lib.sh`
set -e                # Exit on error

function cleanup {
  rm -f *.pem *.csr *.srl *.pfx .index* .serial* .crlnumber*
}

function setup {
  touch .index
  echo 1000 > .serial
  echo 1000 > .crlnumber
}

function generate_server_cert {
  if [ "$#" -ne 1 ]; then
    echo "Usage: generate_server_cert <hostname>"
    exit 1
  fi

  openssl ecparam -name prime256v1 -genkey -noout -out server-key.pem
  openssl req -x509 -key server-key.pem -out server-cert.pem -sha256 -days 365 -subj "/CN=$1"
}

function generate_ca_cert {
  if [ "$#" -ne 1 ]; then
    echo "Usage: generate_ca_cert <name>"
    exit 1
  fi

  openssl ecparam -name prime256v1 -genkey -noout -out ca-key.pem
  openssl req -x509 -key ca-key.pem -out ca-cert.pem -days 365 -subj "/CN=$1"
}

function generate_crl {
  openssl ca -config openssl.cnf -gencrl -out ca-crl.pem
}

function generate_client_cert {
  if [ "$#" -lt 1 ] || [ "$#" -gt 2 ]; then
    echo "Usage: generate_client_cert <name> [<is_user>]"
    exit 1
  fi

  # Add unique suffix if needed
  local suffix=""
  if [ "$2" == "true" ]; then
    suffix="_$USERNAME"
    i=1
    while [ -f client-cert$suffix.pem ]; do
      i=$((i+1))
      suffix="_$USERNAME-$i"
    done
  fi

  # Generate client key and signed certificate
  openssl ecparam -name prime256v1 -genkey -noout -out client-key$suffix.pem
  openssl req -new -key client-key$suffix.pem -out client-cert$suffix.csr -subj "/CN=$1"
  openssl x509 -req -in client-cert$suffix.csr -CA ca-cert.pem -CAkey ca-key.pem -CAcreateserial -out client-cert$suffix.pem -days 365 -extfile ../client-cert.ext

  # Generate PKCS#12 bundle
  if [ "$2" == "true" ]; then
    openssl pkcs12 -export -keypbe NONE -certpbe NONE -nomaciter -passout pass: -inkey client-key$suffix.pem -in client-cert$suffix.pem -out client$suffix.pfx

    # Friendly output
    echo "Files created:"
    echo "- client-cert$suffix.pem (public certificate)"
    echo "- client-key$suffix.pem (private key)"
    echo "- client$suffix.pfx (PKCS#12 bundle for importing)"
  fi
}

function revoke_client_cert {
  if [ "$#" -ne 1 ]; then
    echo "Usage: revoke_client_cert <client-cert.pem>"
    exit 1
  fi

  # Update CRL
  openssl ca -config openssl.cnf -revoke "$1"
  openssl ca -config openssl.cnf -gencrl -out ca-crl.pem
}

function reload_nginx {
  # Reload nginx from inside or outside the container
  COMPOSE_INSTALLED=$(docker compose version > /dev/null 2>&1; echo $?)
  if [ "$COMPOSE_INSTALLED" -eq 0 ]; then
    COMPOSE="docker compose"
  elif [ -x "$(command -v docker-compose)" ]; then
    COMPOSE="docker-compose"
  else
    echo "Reloading Nginx..."	
    nginx -s reload
    exit 0
  fi

  NGINX=$($COMPOSE ps -q nginx) || true
  if [ -n "$NGINX" ]; then
    echo "Reloading Nginx with '$COMPOSE'..."	
    docker exec "$NGINX" nginx -s reload
  else
    echo "Nginx container not found. Cannot reload automatically."
    exit 1
  fi
}
