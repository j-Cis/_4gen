# _4gen — uruchamianie w WASM

Te pliki przygotowują stronę i polecenia Trunk do zbudowania oraz uruchomienia aplikacji w przeglądarce.

## Pliki

- `index.html` — strona z płótnem aplikacji eframe.
- `Trunk.toml` — konfiguracja kompilacji i serwera HTTP.
- `scripts/run-wasm.ps1` — uruchamia serwer developerski i otwiera przeglądarkę.
- `scripts/build-wasm.ps1` — tworzy wersję produkcyjną w katalogu `dist/`.

## Instalacja Trunk

W PowerShell, jednorazowo:

```powershell
cargo install --locked trunk
rustup target add wasm32-unknown-unknown
```

## Uruchomienie

Skopiuj pliki do katalogu głównego repozytorium _4gen, zachowując strukturę katalogów, a następnie:

```powershell
.\scripts\run-wasm.ps1
```

Aplikacja będzie serwowana pod adresem `http://127.0.0.1:8080`.

## Budowanie

```powershell
.\scripts\build-wasm.ps1
```

Wynik trafi do `dist/`.

## Ważne: punkt wejścia Rust

Trunk buduje istniejący cel binarny projektu. Kod uruchamiający eframe musi mieć gałąź WASM, która uruchamia `eframe::WebRunner` na elemencie canvas `the_canvas_id`. Te pliki konfiguracyjne nie zastępują tego kodu. Przed uznaniem aplikacji za gotową do WASM należy zintegrować punkt wejścia z istniejącym `src/main.rs` i sprawdzić pełny build.
