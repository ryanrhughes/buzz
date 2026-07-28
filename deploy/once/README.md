# Buzz on ONCE — all-in-one image

Single-container Buzz (relay + Postgres 17 + Redis + MinIO under s6-overlay)
for the [ONCE](https://once.com) self-hosting platform. See the deployment
plan: https://a.mosaic.heyoodle.com/a/buzz/once-deployment-plan/

## Build

    docker build -t buzz-relay-local .
    docker build -t buzz-once -f deploy/once/Dockerfile --build-arg RELAY_IMAGE=buzz-relay-local deploy/once

## Deploy

    once deploy <image> --host buzz.example.com \
      --env BUZZ_DOMAIN=buzz.example.com \
      --env RELAY_OWNER_PUBKEY=<owner 64-hex pubkey>

Everything else is generated on first boot into `/storage/config/buzz.env`
(relay identity key, HMAC secret, internal service credentials — back this
file up; it is inside `/storage`, so platform backups include it).

Defaults baked in: membership-gated relay, NIP-OA agent admission enabled
(`BUZZ_ALLOW_NIP_OA_AUTH=true` — required for agent observer frames), auto
migrations on. Any relay env can be overridden with more `--env` flags.

## Notes

- Health: the relay serves `/up` (ONCE contract) as an alias of `/health`.
- Postgres is pinned to 17; a major bump must ship as a deliberate migration
  release (see the plan's risk register).
- Backups: `/hooks/pre-backup` adds a `pg_dump` to the `/storage` snapshot.
