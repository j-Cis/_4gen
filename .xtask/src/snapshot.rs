use querypath::QueryPath;
use querypath_fmt::{
    CasePrecedence, CharClass, Column, DirWeightDisplay, GroupStrategy, NoExtPriority, NodeGroup,
    Numeration, QueryPathFmt, SameNamePriority, Sorting, StatsTemporal, StatsWeight, UnitSystem,
    WeightPrecision,
};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::SystemTime;
use temporal_fmt::Temporal;

pub fn generate_snapshot() {
    println!("🔍");
    let results = QueryPath::new()
        .scan_at(["./"])
        .keep_parent(true)
        .match_pattern([
            "./{examples|tests|src|scripts|.github|.xtask|.cargo}/**",
            "./{Cargo|Trunk}.toml",
            "./index.html",
            "./assets/**",
            "./{.gitattributes|.gitignore}",
            "!./.xtask/target/**",
            "!./.xtask/Cargo.lock",
            "!./Cargo.lock",
            "!./{target|.git|dist|dump}/**",
        ])
        .run()
        .expect("Błąd skanowania");

    let sorting = Sorting::new()
        .enabled(true)
        .group_strategy(GroupStrategy::Custom(vec![
            NodeGroup::TextFile,
            NodeGroup::Directory,
            NodeGroup::BinaryFile,
        ]))
        .same_name_priority(SameNamePriority::FileFirst)
        .no_ext_priority(NoExtPriority::Above)
        .ignore_leading_dot(true)
        .char_class_order([CharClass::Special, CharClass::Digit, CharClass::Letter])
        .case_precedence(CasePrecedence::Insensitive);

    let fmt = QueryPathFmt::new()
        .name_width(25)
        .path_width(35)
        .column_order_left(vec![])
        .column_order_right(vec![Column::Weight, Column::Temporal, Column::Path])
        .numeration(
            Numeration::new()
                .enabled(true)
                .numerate_dirs(false)
                .numerate_binaries(false)
                .start_from(1),
        )
        .stats_weight(
            StatsWeight::new()
                .enabled(true)
                .unit_system(UnitSystem::Binary)
                .dir_display(DirWeightDisplay::MatchedOnly)
                .precision(WeightPrecision::Tenths),
        )
        .stats_temporal(StatsTemporal::new().enabled(true).pattern("WYYY-WW-D hh:mm:ss"))
        .sorting(sorting);

    println!("🏗️");
    let now = SystemTime::now();
    let now_name = Temporal::format_system_time(now, "WYYY-WWAAA-D-hhmmssttqq");
    let now_head = Temporal::format_system_time(now, "WYYY-WWAAA-D hh:mm:ss.zzz|tt:qq");
    let raport_title = format!("_4GEN (CODE DUMP / SNAPSHOT) v:{now_head}");
    let raport_file_name = format!("_4Gen_{now_name}.md");
    let raport_output_dir = "./dump";

    let tree_rendered = fmt.format(&results);

    // 🛠️ WYCIĄGANIE PRAWDZIWYCH NUMERÓW Z DRZEWA
    let mut file_indices: HashMap<String, usize> = HashMap::new();
    for line in tree_rendered.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() >= 2 {
            if let Ok(idx) = tokens[0].parse::<usize>() {
                if let Some(path) = tokens.last() {
                    file_indices.insert(path.to_string(), idx);
                }
            }
        }
    }

    let mut doc = String::new();

    doc.push_str(&format!("# {}\n\n", raport_title));
    
    doc.push_str("## Structure\n\n```plaintext\n");
    doc.push_str(&tree_rendered);
    doc.push_str("\n```\n\n");

    doc.push_str("## Source Code Content\n\n");

    let mut text_files: Vec<_> = results.files.iter().filter(|f| !f.is_binary).collect();
    
    // 🛠️ SORTOWANIE WEDŁUG NUMERÓW Z DRZEWA, A NIE ALFABETYCZNIE
    text_files.sort_by_key(|f| file_indices.get(&f.path).copied().unwrap_or(usize::MAX));

    let max_size = 524288;

    for file in text_files {
        // Pomijamy plik, jeśli nie dostał numeru w strukturze
        let idx = match file_indices.get(&file.path) {
            Some(i) => i,
            None => continue, 
        };

        doc.push_str(&format!("### [{}] `{}`\n\n", idx, file.path));

        let path_str = file.path.replace("\\", "/");
        if path_str.contains("/assets/") || path_str.starts_with("assets/") || path_str.starts_with("./assets/") {
            doc.push_str("> *(File content skipped: belongs to assets)*\n\n");
        } else if file.size > max_size {
            doc.push_str(&format!(
                "> *(File skipped: size {} B exceeds limit of {} B)*\n\n",
                file.size, max_size
            ));
        } else {
            let full_path = Path::new(&results.execution_dir).join(&file.path);
            match fs::read_to_string(&full_path) {
                Ok(content) => {
                    let ext = full_path
                        .extension()
                        .and_then(|e| e.to_str())
                        .unwrap_or("");
                    let lang = match ext.to_lowercase().as_str() {
                        "rs" => "rust",
                        "toml" => "toml",
                        "md" => "markdown",
                        "json" => "json",
                        "yaml" | "yml" => "yaml",
                        "html" => "html",
                        "css" => "css",
                        "js" => "javascript",
                        "ts" => "typescript",
                        "py" => "python",
                        "sh" | "bash" => "bash",
                        _ => "plaintext",
                    };
                    doc.push_str(&format!("```{}\n{}\n```\n\n", lang, content));
                }
                Err(_) => {
                    doc.push_str("> *(Error reading file content)*\n\n");
                }
            }
        }
    }

    doc.push_str("## End Structure Summary\n\n```plaintext\n");
    doc.push_str(&tree_rendered);
    doc.push_str("\n```\n\n---\n*Generated automatically by 4gen xtask via j-Cis/{querypath,querypath-fmt,temporal-fmt} (crates.io|github.com)*\n");

    if !Path::new(raport_output_dir).exists() {
        if let Err(e) = fs::create_dir_all(raport_output_dir) {
            eprintln!("❌ : {}", e);
            return;
        }
    }

    let target_path = Path::new(raport_output_dir).join(raport_file_name);
    match fs::write(&target_path, doc) {
        Ok(_) => println!("✅ : {}", target_path.display()),
        Err(e) => eprintln!("❌ : {}", e),
    }
}