use rand::{rngs::SmallRng, Rng};

/// Generate a name for a culture
pub fn generate_culture_name(rng: &mut SmallRng) -> String {
    let prefixes = [
        "Ak", "Al", "Am", "An", "Ar", "As", "At", "Ba", "Be", "Bi", "Bo", "Br", "Ca", "Ce", "Ch",
        "Co", "Da", "De", "Di", "Do", "Dr", "El", "Em", "En", "Er", "Es", "Fa", "Fe", "Fi", "Fo",
        "Fr", "Ga", "Ge", "Gi", "Go", "Gr", "Ha", "He", "Hi", "Ho", "Hu", "Il", "Im", "In", "Is",
        "Ka", "Ke", "Ki", "Ko", "Kr", "La", "Le", "Li", "Lo", "Lu", "Ma", "Me", "Mi", "Mo", "Mu",
        "Na", "Ne", "Ni", "No", "Nu", "Or", "Os", "Pa", "Pe", "Pi", "Po", "Pr", "Ra", "Re", "Ri",
        "Ro", "Ru", "Sa", "Se", "Si", "So", "St", "Su", "Ta", "Te", "Th", "Ti", "To", "Tr", "Tu",
        "Ul", "Um", "Un", "Ur", "Us", "Va", "Ve", "Vi", "Vo", "Wa", "We", "Wi", "Wo", "Ya", "Ye",
        "Za", "Ze", "Zi", "Zo",
    ];

    let middles = [
        "ar", "an", "en", "er", "el", "al", "ol", "or", "ur", "ir", "as", "es", "is", "os", "us",
        "at", "et", "it", "ot", "ut", "ra", "re", "ri", "ro", "ru", "la", "le", "li", "lo", "lu",
        "na", "ne", "ni", "no", "nu", "ma", "me", "mi", "mo", "mu", "th", "sh", "ch", "ng", "nk",
        "lt", "nd", "nt", "st", "sk",
    ];

    let suffixes = [
        "ia", "ium", "ans", "ish", "ese", "an", "ean", "ian", "ine", "ite", "oid", "ar", "er",
        "or", "ur", "as", "es", "is", "os", "us", "ax", "ex", "ix", "ox", "ux", "ath", "eth",
        "ith", "oth", "uth", "on", "in", "en", "un",
    ];

    let prefix = prefixes[rng.gen_range(0..prefixes.len())];
    let middle = if rng.gen_bool(0.6) {
        middles[rng.gen_range(0..middles.len())]
    } else {
        ""
    };
    let suffix = suffixes[rng.gen_range(0..suffixes.len())];

    format!("{}{}{}", prefix, middle, suffix)
}

/// Generate a name for a religion
pub fn generate_religion_name(rng: &mut SmallRng) -> String {
    let types = [
        "Way of ", "Path of ", "Faith of ", "Order of ", "Church of ", "Temple of ", "Cult of ",
        "Doctrine of ", "Truth of ", "Light of ", "Word of ", "Law of ",
    ];

    let concepts = [
        "the Sun",
        "the Moon",
        "the Stars",
        "the Sky",
        "the Earth",
        "the Sea",
        "the Wind",
        "Heaven",
        "Light",
        "Truth",
        "Wisdom",
        "Justice",
        "Peace",
        "Power",
        "Nature",
        "the Ancestors",
        "the Eternal",
        "the Divine",
        "the One",
        "Unity",
        "Balance",
        "Harmony",
        "the Sacred",
        "the Holy",
    ];

    if rng.gen_bool(0.4) {
        // Named after concept
        let t = types[rng.gen_range(0..types.len())];
        let c = concepts[rng.gen_range(0..concepts.len())];
        format!("{}{}", t, c)
    } else {
        // Made up name with -ism suffix
        let prefixes = [
            "Ahn", "Bel", "Cel", "Div", "Eos", "Fir", "Gal", "Hel", "Ist", "Jov", "Kos", "Lum",
            "Mal", "Nov", "Orn", "Pax", "Qal", "Ras", "Sol", "Tel", "Urb", "Val", "Wis", "Xen",
            "Yor", "Zen",
        ];
        let suffixes = ["ism", "ity", "ology", "anism", "inism", "oism"];
        let p = prefixes[rng.gen_range(0..prefixes.len())];
        let s = suffixes[rng.gen_range(0..suffixes.len())];
        format!("{}{}", p, s)
    }
}

/// Generate a name for a deity
pub fn generate_deity_name(rng: &mut SmallRng) -> String {
    let prefixes = [
        "Ah", "Al", "Am", "An", "Ar", "As", "Ba", "Be", "Br", "Ch", "Da", "El", "En", "Er", "Ha",
        "He", "Hi", "Ho", "Is", "Ja", "Ka", "Ki", "Ko", "La", "Lo", "Ma", "Me", "Mi", "Mo", "Na",
        "Ne", "No", "Nu", "Om", "Or", "Os", "Pa", "Ra", "Re", "Ri", "Sa", "Se", "Sh", "Si", "So",
        "Ta", "Te", "Th", "Ti", "To", "Ul", "Um", "Un", "Ur", "Va", "Ve", "Vi", "Wa", "Ya", "Za",
    ];

    let middles = [
        "an", "ar", "el", "en", "er", "ir", "ol", "on", "or", "ul", "un", "ur", "am", "em", "im",
        "om", "um", "ath", "eth", "ith", "oth", "uth",
    ];

    let suffixes = [
        "a", "ah", "el", "il", "on", "us", "is", "os", "as", "es", "ar", "er", "ir", "or", "ur",
        "an", "en", "in", "un", "ax", "ex", "ix", "ox", "ux",
    ];

    let p = prefixes[rng.gen_range(0..prefixes.len())];
    let m = if rng.gen_bool(0.5) {
        middles[rng.gen_range(0..middles.len())]
    } else {
        ""
    };
    let s = suffixes[rng.gen_range(0..suffixes.len())];

    format!("{}{}{}", p, m, s)
}

/// Generate a leader name based on culture
pub fn generate_leader_name(rng: &mut SmallRng, culture_name: &str) -> String {
    // Simple first names that work across cultures
    let first_names = [
        "Aric", "Bram", "Cael", "Dorn", "Erik", "Finn", "Gorm", "Hakan", "Ivan", "Jorn", "Kael",
        "Leif", "Marn", "Nils", "Odin", "Pran", "Quin", "Rolf", "Sven", "Tarn", "Ulf", "Varn",
        "Wulf", "Xan", "Yorn", "Zorn", "Aelric", "Baldur", "Cyrus", "Draco", "Edric", "Freyr",
        "Gideon", "Harald", "Ivar", "Jarl", "Kaspar", "Lothar", "Magnus", "Njord", "Olaf", "Ragnar",
        "Sigurd", "Theron", "Ulric", "Viktor", "Woden", "Alaric", "Bjorn", "Conrad", "Darius",
        "Edmund", "Fenris", "Gareth", "Hrolf", "Igor", "Jokul", "Knut", "Ludwig", "Maxim", "Neron",
        "Osric", "Pelias", "Quintus", "Roland", "Stefan", "Torsten", "Ulfric", "Valdur", "Werner",
    ];

    first_names[rng.gen_range(0..first_names.len())].to_string()
}

/// Generate a dynasty name
pub fn generate_dynasty_name(rng: &mut SmallRng, culture_name: &str) -> String {
    let prefixes = [
        "Black", "White", "Red", "Gold", "Silver", "Iron", "Stone", "Storm", "Sun", "Moon",
        "Star", "Fire", "Frost", "Thunder", "Wind", "Sea", "Mountain", "Forest", "River", "Dawn",
        "Dusk", "Night", "Dragon", "Wolf", "Bear", "Eagle", "Lion", "Hawk", "Raven", "Serpent",
    ];

    let suffixes = [
        "shield", "sword", "blade", "helm", "heart", "blood", "bone", "hand", "fist", "eye",
        "crown", "throne", "tower", "keep", "hold", "guard", "ward", "bane", "born", "walker",
        "rider", "slayer", "breaker", "maker", "finder", "seeker",
    ];

    let endings = ["s", "son", "sen", "ing", "ling", "kin", "folk", "dale", "vale", "wood"];

    if rng.gen_bool(0.4) {
        // Compound name
        let p = prefixes[rng.gen_range(0..prefixes.len())];
        let s = suffixes[rng.gen_range(0..suffixes.len())];
        format!("{}{}", p, s)
    } else {
        // Name with ending
        let base_names = [
            "Alden", "Bren", "Carl", "Dorn", "Erwin", "Falk", "Gorm", "Hal", "Isen", "Jorn",
            "Karl", "Lars", "Mort", "Norn", "Orm", "Pren", "Rald", "Sorn", "Tarn", "Varn",
        ];
        let name = base_names[rng.gen_range(0..base_names.len())];
        let ending = endings[rng.gen_range(0..endings.len())];
        format!("{}{}", name, ending)
    }
}

/// Generate a settlement name
pub fn generate_settlement_name(rng: &mut SmallRng, culture_name: &str) -> String {
    let prefixes = [
        "North", "South", "East", "West", "New", "Old", "Great", "Little", "Upper", "Lower",
        "High", "Low", "Far", "Near", "Inner", "Outer", "Black", "White", "Red", "Green",
        "Gold", "Silver", "Iron", "Stone", "Wood", "River", "Lake", "Sea", "Mountain", "Forest",
    ];

    let roots = [
        "bridge", "ford", "port", "haven", "bury", "burgh", "wick", "stead", "ham", "ton",
        "ville", "field", "dale", "vale", "wood", "grove", "cliff", "shore", "mouth", "gate",
        "wall", "tower", "keep", "hold", "hall", "court", "cross", "well", "spring", "brook",
    ];

    let single_names = [
        "Ironforge", "Stormwind", "Ravenhold", "Winterfell", "Sunspear", "Highgarden", "Dragonstone",
        "Silverkeep", "Goldcrest", "Thornwood", "Blackwater", "Redcliff", "Whitehall", "Greenvale",
        "Darkwood", "Lightshore", "Stonebridge", "Irongate", "Frostholm", "Flamekeep", "Mistral",
        "Crystalvale", "Shadowfen", "Brightwater", "Deepholm", "Stonehaven", "Cloudpeak", "Riverrun",
        "Seaguard", "Mountainhome",
    ];

    if rng.gen_bool(0.5) {
        // Single compound name
        single_names[rng.gen_range(0..single_names.len())].to_string()
    } else {
        // Prefix + root
        let p = prefixes[rng.gen_range(0..prefixes.len())];
        let r = roots[rng.gen_range(0..roots.len())];
        format!("{}{}", p, r)
    }
}

/// Generate a nation name
pub fn generate_nation_name(rng: &mut SmallRng, culture_name: &str) -> String {
    let types = [
        "Kingdom of ",
        "Empire of ",
        "Republic of ",
        "Realm of ",
        "Dominion of ",
        "Federation of ",
        "Duchy of ",
        "Principality of ",
        "Confederation of ",
        "Union of ",
        "League of ",
        "Commonwealth of ",
        "",
        "",
        "",
    ];

    let names = [
        "Aldoria",
        "Bravonia",
        "Caldoria",
        "Drakonia",
        "Eldoria",
        "Frostland",
        "Galoria",
        "Havenia",
        "Istoria",
        "Jovenia",
        "Kaldoria",
        "Lumoria",
        "Maldoria",
        "Novaria",
        "Ostoria",
        "Pravia",
        "Questoria",
        "Ravenia",
        "Sylvania",
        "Taldoria",
        "Umbria",
        "Valdoria",
        "Westoria",
        "Xandria",
        "Yenoria",
        "Zandria",
    ];

    let t = types[rng.gen_range(0..types.len())];
    let n = names[rng.gen_range(0..names.len())];

    if t.is_empty() {
        n.to_string()
    } else {
        format!("{}{}", t, n)
    }
}

/// Generate a war name
pub fn generate_war_name(
    rng: &mut SmallRng,
    attacker_name: &str,
    defender_name: &str,
    year: i32,
) -> String {
    let types = [
        ("War", false),
        ("Conflict", false),
        ("Campaign", false),
        ("Crusade", false),
        ("Invasion", false),
        ("Conquest", false),
    ];

    let (war_type, _) = types[rng.gen_range(0..types.len())];

    let patterns = [
        format!("{}-{} {}", attacker_name, defender_name, war_type),
        format!("{} of {}", war_type, year),
        format!("The Great {}", war_type),
        format!("{} {} of {}", attacker_name, war_type, year),
    ];

    patterns[rng.gen_range(0..patterns.len())].clone()
}

/// Generate a treaty name
pub fn generate_treaty_name(
    rng: &mut SmallRng,
    nation_a: &str,
    nation_b: &str,
    settlement_name: &str,
    year: i32,
) -> String {
    let types = ["Treaty", "Accord", "Pact", "Agreement", "Convention"];
    let t = types[rng.gen_range(0..types.len())];

    let patterns = [
        format!("{} of {}", t, settlement_name),
        format!("{}-{} {}", nation_a, nation_b, t),
        format!("{} {} of {}", settlement_name, t, year),
    ];

    patterns[rng.gen_range(0..patterns.len())].clone()
}
