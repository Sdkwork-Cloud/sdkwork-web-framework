# WEB_STORE Database Module

Canonical lifecycle assets for `sdkwork-web-framework` per `DATABASE_FRAMEWORK_SPEC.md`.

- moduleId: `webstore`
- serviceCode: `WEB_STORE`
- tablePrefix: `webstore_`

## Commands

```bash
pnpm run db:validate
pnpm run db:plan
pnpm run db:init
pnpm run db:migrate
pnpm run db:seed
pnpm run db:status
pnpm run db:drift:check
```

## Migration status

Legacy SQL was consolidated into `ddl/baseline/postgres/0001_*_legacy_baseline.sql` for bootstrap review.
Author contract-first tables in `contract/schema.yaml`, then split baseline into versioned `migrations/` pairs.

Imported legacy sources:
- `crates/sdkwork-web-store-sqlx/migrations/001_web_stores.sql`
- `crates/sdkwork-web-store-sqlx/migrations/002_web_rate_limit_bucket.sql`
- `crates/sdkwork-web-store-sqlx/migrations/003_web_audit_event.sql`
- `crates/sdkwork-web-store-sqlx/migrations/004_web_cors_policy.sql`
- `crates/sdkwork-web-store-sqlx/migrations/005_web_rate_limit_policy.sql`
- `crates/sdkwork-web-store-sqlx/migrations/006_web_tenant_runtime_profile.sql`
- `crates/sdkwork-web-store-sqlx/migrations/007_web_control_node.sql`
- `crates/sdkwork-web-store-sqlx/migrations/008_tenant_concurrent_limit.sql`

Runtime services MUST create pools through `sdkwork-database-sqlx` and register `DefaultDatabaseModule` at bootstrap.
