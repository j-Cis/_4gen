#[derive(Clone, Debug)]
pub struct Individual {
    pub id: usize,
    pub first_names: String,               // imiona
    pub last_names: String,                // nazwiska
    pub gender: String,                    // płeć (Mężczyzna/Kobieta/Inna)
    pub father_first_name: Option<String>, // imie ojca
    pub father_last_name: Option<String>,  // nazwisko ojca
    pub mother_first_name: Option<String>, // imie matki
    pub mother_last_name: Option<String>,  // nazwisko matki
    pub personal_note: String,             // przypis do osoby
    pub occupation_status: String,         // zawód/status
    pub birth_date: Option<String>,        // data urodzenia
    pub birth_place: Option<String>,       // miejscowość urodzenia
    pub birth_address: Option<String>,     // adres urodzenia
    pub death_date: Option<String>,        // data śmierci
    pub death_place: Option<String>,       // miejscowość śmierci
    pub death_address: Option<String>,     // adres śmierci
    pub parents_count: usize,              // liczba rodziców
    pub spouses_count: usize,              // liczba małżonków
    pub children_count: usize,             // liczba dzieci
    pub credentials_count: usize,          // ilość poświadczeń (źródeł)
    pub notes_count: usize,                // ilość notatek
    pub last_modified: String,             // ostatnio zmieniono
    pub is_confidential: bool,             // poufny
    pub tags: Vec<String>,                 // etykiety
}

pub fn get_sample_individuals() -> Vec<Individual> {
    vec![
        Individual {
            id: 1,
            first_names: "Jan Adam".into(),
            last_names: "Kowalski".into(),
            gender: "Mężczyzna".into(),
            father_first_name: Some("Piotr".into()),
            father_last_name: Some("Kowalski".into()),
            mother_first_name: Some("Anna".into()),
            mother_last_name: Some("Nowak".into()),
            personal_note: "Zasłużony rzemieślnik".into(),
            occupation_status: "Kowal".into(),
            birth_date: Some("1885-04-12".into()),
            birth_place: Some("Warszawa".into()),
            birth_address: Some("ul. Stara 4".into()),
            death_date: Some("1942-11-03".into()),
            death_place: Some("Warszawa".into()),
            death_address: None,
            parents_count: 2,
            spouses_count: 1,
            children_count: 3,
            credentials_count: 4,
            notes_count: 2,
            last_modified: "2024-03-12 14:30".into(),
            is_confidential: false,
            tags: vec!["Zweryfikowane".into(), "Linia męska".into()],
        },
        Individual {
            id: 2,
            first_names: "Maria Anna".into(),
            last_names: "Wiśniewska (z d. Kowalska)".into(),
            gender: "Kobieta".into(),
            father_first_name: Some("Jan".into()),
            father_last_name: Some("Kowalski".into()),
            mother_first_name: Some("Zofia".into()),
            mother_last_name: Some("Kamińska".into()),
            personal_note: "Brak aktu urodzenia".into(),
            occupation_status: "Nauczycielka".into(),
            birth_date: Some("1910-08-22".into()),
            birth_place: Some("Kraków".into()),
            birth_address: None,
            death_date: None, // Osoba może żyć lub data jest nieznana
            death_place: None,
            death_address: None,
            parents_count: 2,
            spouses_count: 2,
            children_count: 1,
            credentials_count: 1,
            notes_count: 0,
            last_modified: "2024-03-15 09:15".into(),
            is_confidential: true,
            tags: vec!["Do sprawdzenia".into()],
        },
        Individual {
            id: 3,
            first_names: "Brak danych".into(),
            last_names: "Nieznany".into(),
            gender: "Nieznana".into(),
            father_first_name: None,
            father_last_name: None,
            mother_first_name: None,
            mother_last_name: None,
            personal_note: "Znaleziony jedynie wpis w księdze parafialnej".into(),
            occupation_status: "Rolnik".into(),
            birth_date: None,
            birth_place: Some("Gniezno".into()),
            birth_address: None,
            death_date: Some("1805-01-10".into()),
            death_place: Some("Gniezno".into()),
            death_address: None,
            parents_count: 0,
            spouses_count: 1,
            children_count: 0,
            credentials_count: 1,
            notes_count: 5,
            last_modified: "2024-02-28 18:00".into(),
            is_confidential: false,
            tags: vec!["Ślepy zaułek".into()],
        },
    ]
}
