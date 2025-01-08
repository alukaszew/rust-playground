use std::collections::HashMap;
use std::io;

fn main() {
    println!("Choose your language / Wybierz język:");
    println!("1. English");
    println!("2. Polski");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    let language = match choice.trim() {
        "1" => "EN",
        "2" => "PL",
        _ => {
            println!("Invalid choice, defaulting to English.");
            "EN"
        }
    };

    let translations = initialize_translations();

    println!("{}", translate("ask_name", language, &translations));
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    println!("{}", translate("ask_birth_year", language, &translations));
    let mut birth_year = String::new();
    io::stdin().read_line(&mut birth_year).expect("Failed to read input");

    let birth_year: i32 = birth_year.trim().parse().expect("Please enter a valid number");

    let current_year = 2025;
    let age = current_year - birth_year;

    println!(
        "{} {}! {} {}.",
        translate("hello", language, &translations),
        name.trim(),
        translate("you_are", language, &translations),
        age
    );
}

fn initialize_translations() -> HashMap<&'static str, HashMap<&'static str, &'static str>> {
    let mut translations = HashMap::new();

    let mut en = HashMap::new();
    en.insert("ask_name", "What is your name?");
    en.insert("ask_birth_year", "What year were you born?");
    en.insert("hello", "Hello");
    en.insert("you_are", "You are");

    let mut pl = HashMap::new();
    pl.insert("ask_name", "Jak masz na imię?");
    pl.insert("ask_birth_year", "W którym roku się urodziłeś?");
    pl.insert("hello", "Cześć");
    pl.insert("you_are", "Masz");

    translations.insert("EN", en);
    translations.insert("PL", pl);

    translations
}

fn translate<'a>(
    key: &'a str,
    language: &'a str,
    translations: &'a HashMap<&'a str, HashMap<&'a str, &'a str>>,
) -> &'a str {
    translations
        .get(language)
        .and_then(|lang_map| lang_map.get(key))
        .map_or("Translation not found", |v| v)
}