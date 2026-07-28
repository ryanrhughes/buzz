#!/bin/bash
# Shared env for buzz-once services. Sources generated secrets and derives
# relay config from BUZZ_DOMAIN (+ DISABLE_SSL) passed at `once deploy`.
set -a
[ -f /storage/config/buzz.env ] && . /storage/config/buzz.env
set +a

: "${BUZZ_DOMAIN:?BUZZ_DOMAIN must be provided via once deploy --env}"

if [ "${DISABLE_SSL:-false}" = "true" ]; then
  WS_SCHEME=ws; HTTP_SCHEME=http
else
  WS_SCHEME=wss; HTTP_SCHEME=https
fi

export RELAY_URL="${RELAY_URL:-$WS_SCHEME://$BUZZ_DOMAIN}"
export BUZZ_MEDIA_BASE_URL="${BUZZ_MEDIA_BASE_URL:-$HTTP_SCHEME://$BUZZ_DOMAIN/media}"
export BUZZ_MEDIA_SERVER_DOMAIN="${BUZZ_MEDIA_SERVER_DOMAIN:-$BUZZ_DOMAIN}"
export BUZZ_CORS_ORIGINS="${BUZZ_CORS_ORIGINS:-$HTTP_SCHEME://$BUZZ_DOMAIN}"

export DATABASE_URL="postgres://buzz:${POSTGRES_PASSWORD}@127.0.0.1:5432/buzz"
export REDIS_URL="redis://:${REDIS_PASSWORD}@127.0.0.1:6379"
export BUZZ_S3_ENDPOINT="http://127.0.0.1:9000"
export BUZZ_S3_BUCKET="${BUZZ_S3_BUCKET:-buzz-media}"

export BUZZ_BIND_ADDR="0.0.0.0:80"
export BUZZ_HEALTH_PORT=8080
export BUZZ_GIT_REPO_PATH=/storage/git
export BUZZ_AUTO_MIGRATE="${BUZZ_AUTO_MIGRATE:-true}"
export BUZZ_REQUIRE_AUTH_TOKEN="${BUZZ_REQUIRE_AUTH_TOKEN:-true}"
export BUZZ_REQUIRE_RELAY_MEMBERSHIP="${BUZZ_REQUIRE_RELAY_MEMBERSHIP:-true}"
# Field-proven (2026-07-27): agents are admitted ViaOwner via NIP-OA; without
# this flag their observer frames (desktop activity feeds) are rejected.
export BUZZ_ALLOW_NIP_OA_AUTH="${BUZZ_ALLOW_NIP_OA_AUTH:-true}"
export RUST_LOG="${RUST_LOG:-buzz_relay=info,buzz_db=info,buzz_auth=info,buzz_pubsub=info}"
