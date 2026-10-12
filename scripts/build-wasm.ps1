$ErrorActionPreference = "Stop"

$ProjectRoot = if ($args.Count -gt 0) {
    (Resolve-Path $args[0]).Path
} else {
    (Get-Location).Path
}

Set-Location $ProjectRoot

if (-not (Get-Command trunk -ErrorAction SilentlyContinue)) {
    throw "Nie znaleziono Trunk. Zainstaluj: cargo install --locked trunk"
}

rustup target add wasm32-unknown-unknown
trunk build --release