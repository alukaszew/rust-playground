# Age Calculator with Translations

This is a simple **Rust command-line application** that calculates the user's age based on their birth year. It includes a **language selection feature** with support for **English** and **Polish** translations.

## 🚀 Features
- Asks the user for their **name** and **birth year**.
- Calculates the user's **age** based on the current year.
- Supports **two languages**: English and Polish.

## 📋 Requirements
- Rust (https://www.rust-lang.org/tools/install)

## 🛠 Setup and Run
### 1. Clone the repository
```bash
git clone https://github.com/alukaszew/rust-playground
cd rust-playground
```

### 2. Build and run the project
```bash
cargo run
```

## 📄 How It Works
1. The program prompts the user to **select a language**:
   - `1` for English
   - `2` for Polish

2. The user is asked for their **name** and **birth year**.
3. The program calculates the user's **age** and prints a **personalized message** in the selected language.

## 📚 Code Breakdown
The program consists of the following key functions:

- **`initialize_translations()`**: Initializes a `HashMap` containing translations for both languages.
- **`translate(key, language, translations)`**: Fetches the appropriate translation based on the selected language.
- **`main()`**: The main function that handles user input and displays output.



