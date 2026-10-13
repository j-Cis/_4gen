mod snapshot;

use inquire::{Confirm, Select};
use std::env;
use std::fs;
use std::process::Command;

const HISTORY_FILE: &str = ".xtask_history";

fn main() {
    let mut args = env::args().skip(1);
    let arg = args.next();

    let history = read_history();

    let command_id = match arg.as_deref() {
        Some("--last") | Some("r") => {
            history.first().cloned().unwrap_or_else(|| {
                eprintln!("Brak historii. Uruchom najpierw menu: `cargo x`");
                std::process::exit(1);
            })
        }
        Some("--last-list") => {
            if history.is_empty() {
                eprintln!("Historia jest pusta.");
                std::process::exit(1);
            }
            let mut opts = history.clone();
            opts.push("< back".to_string());
            let choice = Select::new("Wybierz polecenie z historii:", opts)
                .prompt()
                .unwrap_or_else(|_| std::process::exit(0));
            
            if choice == "< back" {
                return;
            }
            choice
        }
        _ => {
            println!("\n🚀 4gen TUI");
            match run_interactive_menu(&history) {
                Some(id) => id,
                None => return, // Anulowano
            }
        }
    };

    // Zabezpieczenie: NIE zapisujemy akcji drastycznych/informacyjnych z gita i czyszczenia do historii "last"
    if command_id != "clean" && command_id != "info-ip" && !command_id.starts_with("git-") {
        save_to_history(&command_id, history);
    }
    
    execute_task(&command_id);
}

fn read_history() -> Vec<String> {
    fs::read_to_string(HISTORY_FILE)
        .unwrap_or_default()
        .lines()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_string())
        .collect()
}

fn save_to_history(cmd: &str, mut history: Vec<String>) {
    history.retain(|x| x != cmd);
    history.insert(0, cmd.to_string());
    history.truncate(10);
    let _ = fs::write(HISTORY_FILE, history.join("\n"));
}

fn run_interactive_menu(history: &[String]) -> Option<String> {
    loop {
        let mut lvl1_opts = vec!["info", "save", "run", "build", "git", "clean"];
        
        if !history.is_empty() {
            lvl1_opts.insert(0, "last");
            lvl1_opts.insert(1, "last list");
        }

        let lvl1 = Select::new("Wybierz akcję:", lvl1_opts).prompt().ok()?;

        match lvl1 {
            "last" => return Some(history[0].clone()),
            "last list" => {
                let mut hist_opts = history.to_vec();
                hist_opts.push("< back".to_string());
                if let Ok(choice) = Select::new("Wybierz z historii:", hist_opts).prompt() {
                    if choice != "< back" {
                        return Some(choice);
                    }
                }
            }
            "info" => {
                let opts = vec!["< back", "ip", "check", "clippy", "test", "fmt", "wasm-check", "all"];
                if let Ok(lvl2) = Select::new("Info:", opts).prompt() {
                    if lvl2 != "< back" {
                        return Some(format!("info-{}", lvl2));
                    }
                }
            }
            "save" => {
                let opts = vec!["< back", "code snapshot", "check", "clippy", "test", "fmt", "wasm-check", "all"];
                if let Ok(lvl2) = Select::new("Save:", opts).prompt() {
                    if lvl2 != "< back" {
                        return Some(format!("save-{}", lvl2.replace(" ", "-")));
                    }
                }
            }
            "run" => {
                let mode_opts = vec!["< back", "debug", "release"];
                if let Ok(lvl2) = Select::new("Run mode:", mode_opts).prompt() {
                    if lvl2 != "< back" {
                        let plat_opts = vec!["< back", "desk", "wasm host", "wasm wifi"];
                        if let Ok(lvl3) = Select::new("Platform:", plat_opts).prompt() {
                            if lvl3 != "< back" {
                                return Some(format!("run-{}-{}", lvl2, lvl3.replace(" ", "-")));
                            }
                        }
                    }
                }
            }
            "build" => {
                let mode_opts = vec!["< back", "debug", "release"];
                if let Ok(lvl2) = Select::new("Build mode:", mode_opts).prompt() {
                    if lvl2 != "< back" {
                        let plat_opts = vec!["< back", "desk", "wasm"];
                        if let Ok(lvl3) = Select::new("Platform:", plat_opts).prompt() {
                            if lvl3 != "< back" {
                                return Some(format!("build-{}-{}", lvl2, lvl3));
                            }
                        }
                    }
                }
            }
            "git" => {
                let git_cat_opts = vec!["< back", "safe", "risk", "risk-help"];
                if let Ok(cat) = Select::new("Kategoria Git:", git_cat_opts).prompt() {
                    if cat != "< back" {
                        let sub_opts = match cat {
                            "safe" => vec![
                                "< back",
                                "status (krótki)",
                                "log (ostatnie 10 - graf)",
                                "log (ostatnie 5 z haszami)",
                                "branches (lista gałęzi)",
                                "tags (lista tagów)",
                                "reflog (historie zmian HEAD)",
                            ],
                            "risk" => vec![
                                "< back",
                                "add all (git add .)",
                                "commit quick (quick save)",
                                "push (origin)",
                                "pull (rebase)",
                                "stash (zapisz i ukryj zmiany)",
                            ],
                            "risk-help" => vec![
                                "< back",
                                "undo commit (soft - zachowaj pliki)",
                                "undo commit (hard - usuń zmiany)",
                                "unstage all (git reset)",
                                "stash pop (przywróć zmiany)",
                                "clean untracked (git clean -fd)",
                            ],
                            _ => vec!["< back"],
                        };

                        if let Ok(action) = Select::new("Polecenie Git:", sub_opts).prompt() {
                            if action != "< back" {
                                return Some(format!("git-{}-{}", cat, action.replace(" ", "-")));
                            }
                        }
                    }
                }
            }
            "clean" => return Some("clean".to_string()),
            _ => return None,
        }
    }
}

fn execute_task(id: &str) {
    println!("\n> Wykonuję: {}\n", id);

    match id {
        "clean" => {
            let ans = Confirm::new("⚠️ UWAGA: Czy na pewno chcesz usunąć WSZYSTKIE artefakty kompilacji (cargo clean)?")
                .with_default(false)
                .prompt();
                
            match ans {
                Ok(true) => run_cmd("cargo", &["clean"]),
                _ => println!("Anulowano czyszczenie projektu."),
            }
        }

        "info-ip" => print_ip_reminder(),
        "info-check" => run_cmd("cargo", &["check", "--all-targets"]),
        "info-clippy" => run_cmd("cargo", &["clippy", "--all-targets", "--", "-D", "warnings"]),
        "info-test" => run_cmd("cargo", &["test"]),
        "info-fmt" => run_cmd("cargo", &["fmt", "--all", "--", "--check"]),
        "info-wasm-check" => {
            execute_task("x-wasm-target");
            run_cmd("cargo", &["check", "--target", "wasm32-unknown-unknown"]);
        }
        "info-all" => {
            execute_task("info-fmt"); execute_task("info-check"); execute_task("info-test"); execute_task("info-clippy"); execute_task("info-wasm-check");
        }

        "save-code-snapshot" => snapshot::generate_snapshot(),
        
        "save-check" => execute_task("info-check"),
        "save-clippy" => execute_task("info-clippy"),
        "save-test" => execute_task("info-test"),
        "save-fmt" => execute_task("info-fmt"),
        "save-wasm-check" => execute_task("info-wasm-check"),
        "save-all" => execute_task("info-all"),

        "run-debug-desk" => run_cmd("cargo", &["run"]),
        "run-debug-wasm-host" => {
            execute_task("x-wasm-target");
            run_cmd("trunk", &["serve", "--address", "127.0.0.1", "--port", "8080", "--open"]);
        }
        "run-debug-wasm-wifi" => {
            execute_task("x-wasm-target");
            print_ip_reminder();
            run_cmd("trunk", &["serve", "--address", "0.0.0.0", "--port", "8080"]);
        }

        "run-release-desk" => run_cmd("cargo", &["run", "--release"]),
        "run-release-wasm-host" => {
            execute_task("x-wasm-target");
            run_cmd("trunk", &["serve", "--release", "--address", "127.0.0.1", "--port", "8080", "--open"]);
        }
        "run-release-wasm-wifi" => {
            execute_task("x-wasm-target");
            print_ip_reminder();
            run_cmd("trunk", &["serve", "--release", "--address", "0.0.0.0", "--port", "8080"]);
        }

        "build-debug-desk" => run_cmd("cargo", &["build"]),
        "build-debug-wasm" => {
            execute_task("x-wasm-target");
            run_cmd("trunk", &["build"]);
        }
        "build-release-desk" => run_cmd("cargo", &["build", "--release"]),
        "build-release-wasm" => {
            execute_task("x-wasm-target");
            run_cmd("trunk", &["build", "--release"]);
        }

        // --- GIT SAFE ---
        "git-safe-status-(krótki)" => run_cmd("git", &["status", "-s"]),
        "git-safe-log-(ostatnie-10---graf)" => run_cmd("git", &["log", "--oneline", "--graph", "--decorate", "-n", "10"]),
        "git-safe-log-(ostatnie-5-z-haszami)" => run_cmd("git", &["log", "-n", "5", "--format=format:%h - %an, %ar : %s"]),
        "git-safe-branches-(lista-gałęzi)" => run_cmd("git", &["branch", "-a", "-v"]),
        "git-safe-tags-(lista-tagów)" => run_cmd("git", &["tag", "-l", "-n1"]),
        "git-safe-reflog-(historie-zmian-HEAD)" => run_cmd("git", &["reflog", "-n", "10"]),

        // --- GIT RISK ---
        "git-risk-add-all-(git-add-.)" => {
            if confirm_action("Dodać WSZYSTKIE zmodyfikowane pliki do indeksu (git add .)?") {
                run_cmd("git", &["add", "."]);
            }
        }
        "git-risk-commit-quick-(quick-save)" => {
            if confirm_action("Wykonać szybki commit: 'refactor: replace cargo-make with pure Rust .xtask TUI'?") {
                run_cmd("git", &["commit", "-m", "refactor: replace cargo-make with pure Rust .xtask TUI"]);
            }
        }
        "git-risk-push-(origin)" => {
            if confirm_action("Wysłać zocommitowane zmiany na zdalne repozytorium (git push)?") {
                run_cmd("git", &["push"]);
            }
        }
        "git-risk-pull-(rebase)" => {
            if confirm_action("Pobrać zmiany ze zdalnego repozytorium (git pull --rebase)?") {
                run_cmd("git", &["pull", "--rebase"]);
            }
        }
        "git-risk-stash-(zapisz-i-ukryj-zmiany)" => {
            if confirm_action("Schować niezocommitowane zmiany na stos (git stash)?") {
                run_cmd("git", &["stash"]);
            }
        }

        // --- GIT RISK-HELP ---
        "git-risk-help-undo-commit-(soft---zachowaj-pliki)" => {
            if confirm_action("Cofnąć OSTATNI commit zachowując pliki w roboczym katalogu (git reset --soft HEAD~1)?") {
                run_cmd("git", &["reset", "--soft", "HEAD~1"]);
            }
        }
        "git-risk-help-undo-commit-(hard---usuń-zmiany)" => {
            if confirm_action("⚠️ BEZPOWROTANIE usunąć ostatni commit i wszystkie nieskomitowane zmiany (git reset --hard HEAD~1)?") {
                run_cmd("git", &["reset", "--hard", "HEAD~1"]);
            }
        }
        "git-risk-help-unstage-all-(git-reset)" => {
            if confirm_action("Wycofać pliki ze strefy oczekiwania / staging (git reset)?") {
                run_cmd("git", &["reset"]);
            }
        }
        "git-risk-help-stash-pop-(przywróć-zmiany)" => {
            if confirm_action("Przywrócić ostatnio schowane zmiany ze stosu (git stash pop)?") {
                run_cmd("git", &["stash", "pop"]);
            }
        }
        "git-risk-help-clean-untracked-(git-clean--fd)" => {
            if confirm_action("⚠️ BEZPOWROTANIE usunąć wszystkie niośledzone pliki i katalogi (git clean -fd)?") {
                run_cmd("git", &["clean", "-fd"]);
            }
        }

        "x-wasm-target" => run_cmd("rustup", &["target", "add", "wasm32-unknown-unknown"]),

        _ => eprintln!("Nieznane zadanie: {}", id),
    }
}

fn confirm_action(msg: &str) -> bool {
    Confirm::new(msg)
        .with_default(false)
        .prompt()
        .unwrap_or(false)
}

fn print_ip_reminder() {
    println!("-------------------------------------------");
    println!(" Sprawdzam IP dla LAN (WASM WIFI)...");
    let output = Command::new("cmd").args(["/C", "ipconfig | findstr IPv"]).output();
    if let Ok(out) = output {
        print!("{}", String::from_utf8_lossy(&out.stdout));
    }
    println!(" Uruchom na telefonie: http://<TWOJ_IP>:8080");
    println!("-------------------------------------------\n");
}

fn run_cmd(program: &str, args: &[&str]) {
    let mut child = Command::new(program).args(args).spawn().expect("Nie udało się uruchomić procesu");
    let status = child.wait().expect("Błąd oczekiwania na proces");
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}