#[derive(Clone, Debug)]
pub struct Event {
    pub id: usize,
    pub event_type: String,   // rodzaj (np. Ślub, Narodziny, Chrzest, Pogrzeb)
    pub date: Option<String>, // data
    pub place: Option<String>, // miejsce (miejscowość)
    pub address: Option<String>, // adres
    pub description: String,  // opis
    pub participants: Vec<String>, // uczestnicy (np. lista imion/ról)
    pub credentials_count: usize, // ilość poświadczeń (źródeł)
    pub notes_count: usize,   // ilość notatek
    pub participants_count: usize, // ilość uczestników
    pub last_modified: String, // ostatnio zmieniono
    pub is_confidential: bool, // poufny
    pub tags: Vec<String>,    // etykiety
}

pub fn get_sample_events() -> Vec<Event> {
    vec![
        Event {
            id: 1,
            event_type: "Ślub".into(),
            date: Some("1910-10-15".into()),
            place: Some("Kraków".into()),
            address: Some("Kościół Mariacki, Rynek Główny".into()),
            description:
                "Ślub Jana Kowalskiego i Marii Nowak. Wesele odbyło się w pobliskiej karczmie."
                    .into(),
            participants: vec![
                "Jan Kowalski (Pan Młody)".into(),
                "Maria Nowak (Panna Młoda)".into(),
                "Piotr Kowalski (Świadek)".into(),
            ],
            credentials_count: 2,
            notes_count: 1,
            participants_count: 3,
            last_modified: "2024-03-20 10:15".into(),
            is_confidential: false,
            tags: vec!["Sakrament".into(), "Kluczowe".into()],
        },
        Event {
            id: 2,
            event_type: "Narodziny".into(),
            date: Some("1885-04-12".into()),
            place: Some("Warszawa".into()),
            address: None,
            description: "Narodziny pierworodnego syna.".into(),
            participants: vec![
                "Jan Adam Kowalski (Dziecko)".into(),
                "Anna Nowak (Matka)".into(),
            ],
            credentials_count: 1,
            notes_count: 0,
            participants_count: 2,
            last_modified: "2024-02-15 08:30".into(),
            is_confidential: false,
            tags: vec!["Akt urodzenia".into()],
        },
        Event {
            id: 3,
            event_type: "Pogrzeb".into(),
            date: Some("1942-11-06".into()),
            place: Some("Warszawa".into()),
            address: Some("Cmentarz Powązkowski".into()),
            description: "Ceremonia pogrzebowa w gronie najbliższej rodziny.".into(),
            participants: vec![
                "Jan Adam Kowalski (Zmarły)".into(),
                "Maria Wiśniewska (Córka)".into(),
            ],
            credentials_count: 3,
            notes_count: 2,
            participants_count: 12, // Zadeklarowana większa liczba uczestników, mimo wymienienia tylko kluczowych wyżej
            last_modified: "2024-03-22 18:45".into(),
            is_confidential: true,
            tags: vec!["II Wojna Światowa".into()],
        },
    ]
}
