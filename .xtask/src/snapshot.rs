use querypath::QueryPath;
use querypath_fmt::{
	CasePrecedence, CharClass, Column, DirWeightDisplay, GroupStrategy, NoExtPriority, NodeGroup,
	QueryPathBundle, QueryPathFmt, SameNamePriority, Sorting, StatsTemporal, StatsWeight,
	UnitSystem, WeightPrecision,
};
use querypath_snapshot::Snapshot;

pub fn generate_snapshot() {
	println!("🔍");
	// 1. Skanowanie systemu plików
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

	// 2. Strategia sortowania
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

	// 3. Złożenie, posortowanie i nadanie atomowej numeracji w drzewie
	let idn = QueryPathBundle::new()
		.queueing_before_numerate(sorting)
		.numerate_dirs(false)
		.numerate_binaries(false)
		.numerate_skip(["./assets/**", "*.svg"])
		.numerate_start_from(1)
		.apply_to(&results);

	// 4. Konfiguracja widoku kolumn i stylów drzewa
	let fmt = QueryPathFmt::new()
		.name_width(25)
		.path_width(35)
		.column_order_left(vec![])
		.column_order_right(vec![Column::Weight, Column::Temporal, Column::Path])
		.stats_weight(
			StatsWeight::new()
				.enabled(true)
				.unit_system(UnitSystem::Binary)
				.dir_display(DirWeightDisplay::MatchedOnly)
				.precision(WeightPrecision::Tenths),
		)
		.stats_temporal(StatsTemporal::new().enabled(true).pattern("WYYY-WW-D hh:mm:ss"));

	// 5. Deklaratywne wygenerowanie i zapis migawki (Snapshot)
	println!("🏗️");
	let snapshot = Snapshot::new()
		.title("_4GEN (CODE DUMP / SNAPSHOT) v:")
		.title_sufix_date_fmt("WYYY-WW D::hh:mm:ss:tt:qq")
		.name("_4Gen_")
		.name_sufix_date_fmt("WYYYWW-Dhhmm-ssttqq")
		.output_dir("./dump")
		.max_single_file_size(524288) // 512 KiB
		.content_skip(["./assets/**", "*.svg"])
		.add_footer_summary_setups(true)
		.add_fotter_customize_text(
			"\n---\n*Generated automatically by 4gen xtask via j-Cis/{querypath,querypath-fmt,querypath-snapshot} (crates.io|github.com)*\n",
		);

	match snapshot.generate_and_save(&results, &idn, &fmt) {
		Ok(path) => println!("✅ : {}", path.display()),
		Err(e) => eprintln!("❌ : {}", e),
	}
}