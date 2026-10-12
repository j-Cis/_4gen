$ErrorActionPreference = "Stop"

# Run from the repository root, or pass the repository path as the first argument.
$ProjectRoot = if ($args.Count -gt 0) {
    (Resolve-Path $args[0]).Path
} else {
    (Get-Location).Path
}

Set-Location $ProjectRoot

if (-not (Get-Command trunk -ErrorAction SilentlyContinue)) {
    throw @"
Nie znaleziono polecenia 'trunk'.
Zainstaluj Trunk poleceniem:
    cargo install --locked trunk
Następnie ponownie uruchom:
    .\scripts\run-wasm.ps1
"@
}

rustup target add wasm32-unknown-unknown
trunk serve --address 127.0.0.1 --port 8080 --open