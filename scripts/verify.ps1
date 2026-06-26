# SDKWork web-framework verification entrypoint
$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $true

Set-Location (Split-Path $PSScriptRoot -Parent)

Remove-Item Env:SDKWORK_WEB_FRAMEWORK_ENV -ErrorAction SilentlyContinue

Write-Host "Running cargo test --workspace..."
cargo test --workspace
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "Running architecture tests..."
cargo test -p sdkwork-web-architecture-tests
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "Running bootstrap integration tests (contract fallback, production assembly)..."
cargo test -p sdkwork-web-bootstrap --test integration
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "Running openapi authority contract tests..."
cargo test -p sdkwork-routes-web-framework-backend-api --test openapi_authority
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "Running admin route manifest contract tests..."
cargo test -p sdkwork-routes-web-framework-backend-api --test routes_contract
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "Running admin-api readiness integration test..."
cargo test -p sdkwork-web-bootstrap --features admin-api --test admin_api_readiness
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "Running admin-server control-plane assembly tests..."
cargo test -p sdkwork-web-admin-server
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

if (Get-Command node -ErrorAction SilentlyContinue) {
    Write-Host "Checking PC admin operations.ts generation drift..."
    node scripts/generate-pc-admin-operations.mjs --check
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

    Write-Host "Running database framework contract test..."
    node tests/contract/database-framework.contract.test.mjs
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

    Write-Host "Running PC admin operations contract test..."
    node tests/contract/pc-admin-operations.contract.test.mjs
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

    Write-Host "Running production rollout contract test..."
    node tests/contract/production-rollout.contract.test.mjs
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

    Write-Host "Running release evidence contract tests..."
    node tests/contract/release-evidence.contract.test.mjs
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    node tests/contract/adoption-evidence.contract.test.mjs
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    node scripts/collect-release-evidence.mjs
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
}

$pcApp = Join-Path (Get-Location) "apps/sdkwork-web-framework-pc"
if (Test-Path (Join-Path $pcApp "package.json")) {
    Write-Host "Running PC admin console verify..."
    Push-Location $pcApp
    if (-not (Test-Path "node_modules")) {
        npm ci
        if ($LASTEXITCODE -ne 0) { Pop-Location; exit $LASTEXITCODE }
    } elseif (-not (Test-Path "node_modules/@playwright/test")) {
        npm install
        if ($LASTEXITCODE -ne 0) { Pop-Location; exit $LASTEXITCODE }
    }
    npm run verify
    if ($LASTEXITCODE -ne 0) { Pop-Location; exit $LASTEXITCODE }
    Pop-Location

    Write-Host "Running PC admin build smoke test..."
    node tests/contract/pc-admin-build.smoke.test.mjs
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

    Write-Host "Running PC admin Playwright E2E smoke..."
    Push-Location $pcApp
    npm run test:e2e
    if ($LASTEXITCODE -ne 0) { Pop-Location; exit $LASTEXITCODE }
    Pop-Location

    Write-Host "Building PC admin console for integration E2E..."
    node tests/contract/pc-admin-e2e-build.contract.test.mjs
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

    Write-Host "Running PC admin Playwright integration E2E..."
    Push-Location $pcApp
    npm run test:e2e:integration
    if ($LASTEXITCODE -ne 0) { Pop-Location; exit $LASTEXITCODE }
    Pop-Location
}

if ($env:SDKWORK_REDIS_TEST_URL) {
    Write-Host "Running live Redis integration tests..."
    cargo test -p sdkwork-web-store-redis --test redis_live -- --ignored
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
}

Write-Host "Running release pipeline benchmark..."
& (Join-Path $PSScriptRoot "benchmark-pipeline.ps1")
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "Running clippy..."
cargo clippy --workspace -- -D warnings
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

exit 0
