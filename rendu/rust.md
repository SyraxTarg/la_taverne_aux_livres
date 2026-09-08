installation rust 
https://rust-lang.org/fr/learn/get-started/


créer un nouveau projet rusrt
cargo new <projet>

run le projet
cargo run


ferrissay

```rust
use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}
```

le runtime utiilsé par rust est Tokio


cargo add <crate>
un crate c une dependance on l'ajoute au cargo.toml comme ceci


il faut un fichier mod.rs dans chaque dossier pour déclarer les modules qui s'y trouvent
```rust
pub mod <nom du fichier du module>


ducumentation de l'api google livres
https://developers.google.com/books/docs/v1/using?hl=fr