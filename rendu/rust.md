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


Why Rust for Machine Learning?
Rust is becoming popular in the machine learning community for several reasons:

Memory Safety: Rust prevents bugs like null pointer errors and buffer overflows. This makes machine learning code safer.
High Performance: Rust runs as fast as C and C++. It has no garbage collection, so machine learning models run efficiently.
Concurrency: Rust supports parallel computing. It helps machine learning models run faster on multi-core processors.
Interoperability: Rust works well with C and Python. You can use existing machine learning frameworks while getting Rust’s speed benefits.


La similarité cosinus est un indicateur largement utilisé pour déterminer la similarité de deux points de données en fonction de la direction dans laquelle ils pointent, et non de leur longueur ni de leur taille. Elle est particulièrement efficace dans les espaces à haute dimension, où les mesures traditionnelles axées sur la distance peuvent s’avérer difficiles à mettre en œuvre.
 
Pour calculer la similarité cosinus, il faut mesurer le cosinus de l’angle (thêta) entre deux vecteurs non nuls dans un espace de produit interne. Cette mesure génère un score de similarité cosinus. Les valeurs de similarité cosinus varient de -1 à 1 :

Un score de similarité cosinus de 1 indique que les vecteurs pointent dans la même direction.
Un score de similarité cosinus de 0 indique que les vecteurs sont orthogonaux, ce qui signifie qu’ils n’ont aucune similarité directionnelle.
Un score de similarité cosinus de -1 indique que les vecteurs pointent dans des directions complètement opposées.
C’est un peu comme comparer des flèches : si elles pointent dans la même direction, elles sont très similaires. Celles à angle droit sont sans rapport, et les flèches pointant dans des directions opposées sont dissemblables.



en gros a chaque fois qu'un user note un livre, on attribu au livre un vecteur (une suite de nombre déterminé sur la base de ....) et dans le profil de l'utilisateur on fait une moyenne de tous ces vecteurs ensuite on calcule la similarité entre le vectuer de l'utilisateur et celui des autres livres

on va aller chercher par exemple les 5 catégories les plus aimées, requeter l'api google sur ces 5 catégories puis rechercher les livres avec les plus gros vecteurs correspondants

1. Pourquoi tu n'as pas besoin d'entraîner le modèle linguistique ?
On utilise le principe du Transfer Learning avec un modèle pré-entraîné (Pre-trained Model comme MiniLM ou BERT).

Qui l'a entraîné ? De grands laboratoires (Hugging Face, Google, etc.) ont déjà fait tourner des serveurs surpuissants pendant des semaines sur des millions de textes, livres et articles Wikipedia.
Le résultat ? Un fichier de « poids » très léger (environ 80 à 120 Mo).
En Rust : Quand tu utilises une crate comme fastembed, elle télécharge automatiquement ce petit fichier au premier démarrage et le garde en cache local. Le « cerveau » du modèle comprend déjà la langue française et anglaise dès la première seconde !


2. Alors, qu'est-ce qui « apprend » dans ton application ?
Ce n'est pas le réseau de neurones qui se ré-entraîne, c'est le profil de l'utilisateur qui s'ajuste dynamiquement en temps réel.

On appelle ça de l'apprentissage continu (Online Learning) :

Marc s'inscrit : son profil est vide.
Marc met 5/5 à un livre de SF : son profil devient le vecteur de ce livre.
Deux semaines plus tard, Marc met 5/5 à un roman policier : ton backend recalcule instantanément la moyenne de ses goûts.

Le calcul en Rust (durée : 0,0001 seconde !) :
Chaque fois qu'un utilisateur ajoute ou modifie une note, tu calcules simplement la moyenne pondérée des vecteurs des livres qu'il a aimés
