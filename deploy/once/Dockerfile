# syntax=docker/dockerfile:1.7
#
# All-in-one Buzz image for the ONCE platform (https://once.com):
# relay + Postgres 17 + Redis + MinIO supervised by s6-overlay in one
# container, HTTP on :80, state under /storage, healthcheck on /up.
#
# Build (from repo root):
#   docker build -t buzz-relay-local .
#   docker build -t buzz-once -f deploy/once/Dockerfile --build-arg RELAY_IMAGE=buzz-relay-local deploy/once
#
# Deploy:
#   once deploy <image> --host <domain> \
#     --env BUZZ_DOMAIN=<domain> --env RELAY_OWNER_PUBKEY=<64-hex>
ARG RELAY_IMAGE=buzz-relay-local
FROM ${RELAY_IMAGE}

USER root

ARG S6_OVERLAY_VERSION=3.2.0.2
ARG TARGETARCH=amd64

# s6-overlay (supervisor, PID 1)
ADD https://github.com/just-containers/s6-overlay/releases/download/v${S6_OVERLAY_VERSION}/s6-overlay-noarch.tar.xz /tmp/s6-noarch.tar.xz
ADD https://github.com/just-containers/s6-overlay/releases/download/v${S6_OVERLAY_VERSION}/s6-overlay-x86_64.tar.xz /tmp/s6-arch.tar.xz
RUN apt-get update \
    && apt-get install -y --no-install-recommends xz-utils gnupg lsb-release \
    && tar -C / -Jxpf /tmp/s6-noarch.tar.xz \
    && tar -C / -Jxpf /tmp/s6-arch.tar.xz \
    && rm /tmp/s6-*.tar.xz

# Postgres 17 (pgdg), Redis
RUN install -d /usr/share/postgresql-common/pgdg \
    && curl -fsSL https://www.postgresql.org/media/keys/ACCC4CF8.asc \
        -o /usr/share/postgresql-common/pgdg/apt.postgresql.org.asc \
    && echo "deb [signed-by=/usr/share/postgresql-common/pgdg/apt.postgresql.org.asc] https://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" \
        > /etc/apt/sources.list.d/pgdg.list \
    && apt-get update \
    && apt-get install -y --no-install-recommends postgresql-17 redis-server \
    && rm -rf /var/lib/apt/lists/*

# MinIO server + mc client (pinned static binaries)
RUN curl -fsSL https://dl.min.io/server/minio/release/linux-${TARGETARCH}/minio -o /usr/local/bin/minio \
    && curl -fsSL https://dl.min.io/client/mc/release/linux-${TARGETARCH}/mc -o /usr/local/bin/mc \
    && chmod 0755 /usr/local/bin/minio /usr/local/bin/mc

COPY rootfs/ /

RUN chmod 0755 /hooks/pre-backup /hooks/post-restore \
        /usr/local/lib/buzz-once/*.sh \
        /etc/s6-overlay/s6-rc.d/*/run 2>/dev/null || true; \
    chmod 0755 /etc/s6-overlay/s6-rc.d/buzz-init/up.sh /etc/s6-overlay/s6-rc.d/minio-init/up.sh

# ONCE contract: app speaks HTTP on 80; relay health sidecar port stays internal.
EXPOSE 80
ENV S6_BEHAVIOUR_IF_STAGE2_FAILS=2 \
    S6_CMD_WAIT_FOR_SERVICES_MAXTIME=120000

ENTRYPOINT ["/init"]
CMD []
