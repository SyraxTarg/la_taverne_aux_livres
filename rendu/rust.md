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


générer une clé secrete pour l'api (faut que ce soit la meme dans le front)
openssl rand -hex 32


En Rust, si tu mets un point-virgule à la fin de la dernière ligne d'une fonction, elle ne retourne rien (elle renvoie le type unité ()). Or, la signature de tes fonctions indique qu'elles doivent retourner un Result<user::Model, DbErr>.
