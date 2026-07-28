#!/command/with-contenv bash
set -euo pipefail
. /usr/local/lib/buzz-once/env.sh
for i in $(seq 1 60); do
  if curl -fsS http://127.0.0.1:9000/minio/health/ready >/dev/null 2>&1; then break; fi
  sleep 1
done
export MC_CONFIG_DIR=/tmp/mc
/usr/local/bin/mc alias set local http://127.0.0.1:9000 "$BUZZ_S3_ACCESS_KEY" "$BUZZ_S3_SECRET_KEY" >/dev/null
/usr/local/bin/mc mb --ignore-existing "local/$BUZZ_S3_BUCKET" >/dev/null
echo "minio-init: bucket $BUZZ_S3_BUCKET ready"
