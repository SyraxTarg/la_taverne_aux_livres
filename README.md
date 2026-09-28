# 📚 La Taverne aux Livres

Bienvenue sur **La Taverne aux Livres**, une application web moderne pour explorer, suivre et noter vos lectures, échanger avec d'autres passionnés et découvrir de nouvelles pépites littéraires grâce à un moteur de recommandation propulsé par l'Intelligence Artificielle (Machine Learning).

---

## 🌟 Fonctionnalités

- **📖 Recherche & Découverte** : Exploration du catalogue mondial via l'API Google Books (recherche par titre, auteur, catégories, pagination).
- **⭐ Suivi de Lecture & Notations** : Enregistrement des livres lus avec attribution d'une note (de 1 à 5 étoiles) et statistiques globales par livre.
- **💬 Commentaires & Discussions** : Espace d'échange sous chaque livre avec gestion des réponses hiérarchiques (commentaires imbriqués).
- **🤖 Recommandations Intelligentes (Machine Learning)** :
  - Analyse sémantique des préférences de lecture avec le modèle NLP pré-entraîné **Multilingual-E5-small** via **FastEmbed** (ONNX Runtime).
  - Modélisation du profil utilisateur et scoring des candidats par **similarité cosinus**.
  - Filtrage automatique des genres génériques et priorisation des livres avec résumés détaillés.
- **🔒 Sécurité & Authentification** :
  - Authentification par jeton **JWT**.
  - Hachage sécurisé des mots de passe avec **Argon2id**.
  - Gestion des rôles.
- **📑 Documentation Interactive** : Swagger UI OpenAPI intégré pour tester directement toutes les routes de l'API.

---

## 🛠️ Stack Technique

### Backend
- **Langage** : Rust (Edition 2024)
- **Framework Web** : [Axum 0.8](https://github.com/tokio-rs/axum) (asynchrone, Tokio)
- **ORM & Base de données** : [Sea-ORM 2.0](https://www.sea-ql.org/SeaORM/) avec **PostgreSQL 16**
- **Machine Learning / IA** : [FastEmbed 4](https://github.com/Anush008/fastembed-rs) (modèle `intfloat/multilingual-e5-small`)
- **Documentation API** : [Utoipa](https://github.com/juhaku/utoipa) & Swagger UI

### Frontend
- **Framework** : [Astro 7](https://astro.build/) (Node.js 22+)
- **Architecture** : Rendu orienté composants performant

### DevOps & Conteneurisation
- **Docker & Docker Compose** (environnements développement et production)
- **CI/CD** : GitHub Actions (build des images, push sur Docker Hub et déploiement)

---

## 📂 Structure du Projet

```text
la_taverne_aux_livres/
├── .github/workflows/       # Pipelines CI/CD (GitHub Actions)
├── backend/                 # API Rust (Axum, Sea-ORM, FastEmbed)
│   ├── src/
│   │   ├── api/             # Contrôleurs, DTOs, entités, repositories, services
│   │   ├── proxy/           # Intégration de l'API Google Books
│   │   └── main.rs          # Point d'entrée, initialisation du modèle ML et routes
│   ├── dockerfile           # Image Docker multi-stage pour le backend
│   └── Cargo.toml           # Dépendances Rust
├── frontend/                # Application Web Astro
│   ├── src/                 # Pages, composants, layouts, styles
│   └── package.json         # Dépendances Node.js / Astro
├── docker-compose.yaml      # Configuration Docker Compose locale
├── docker-compose.production.yaml # Configuration Docker Compose pour déploiement distant
└── README.md                # Documentation générale du projet
```

---

## 🚀 Démarrage Rapide

### Prérequis
- [Docker](https://docs.docker.com/get-docker/) et [Docker Compose](https://docs.docker.com/compose/) installés.
- Une clé API [Google Books](https://developers.google.com/books).

### 1. Configuration des variables d'environnement

Créez le fichier de configuration du backend à partir de l'exemple :

```bash
cp backend/.env.example backend/.env
```

Éditez `backend/.env` avec vos valeurs :

```env
# Clé et URL de l'API Google Books
GOOGLE_BOOKS_API_KEY=votre_cle_api_google_books
GOOGLE_BOOKS_API_URL="https://www.googleapis.com/books/v1"

# Base de données PostgreSQL
DATABASE_URL=postgresql://postgres:root@postgres:5432/tavern

# Secret pour signer les JWT
JWT_SECRET=un_secret_tres_long_et_securise

# Serveur Backend
HOST="0.0.0.0"
PORT="3000"
```

Pour la production ou l'utilisation du fichier compose de production, configurez également la racine :
```bash
cp .env.sample .env
```
Indiquez la version de votre image applicative dans `.env` :
```env
APP_VERSION=v1.0.0
```

---

### 2. Lancement avec Docker Compose (Recommandé)

Démarrez l'ensemble des services (PostgreSQL, Backend Rust, Frontend Astro) en une seule commande :

```bash
docker compose up --build -d
```

Les services sont alors accessibles :
- **Frontend** : [http://localhost:4321](http://localhost:4321)
- **API Backend** : [http://localhost:3000](http://localhost:3000)
- **Documentation Swagger UI** : [http://localhost:3000/swagger-ui](http://localhost:3000/swagger-ui)
- **PostgreSQL** : `localhost:5445` (utilisateur : `postgres`, base : `tavern`, mot de passe : `root`)

Pour arrêter les services :
```bash
docker compose down
```

---

### 3. Exécution en Local (Sans Docker)

Si vous souhaitez exécuter le backend et le frontend directement sur votre machine :

#### A. Démarrer PostgreSQL
Assurez-vous d'avoir une instance PostgreSQL locale ou lancez uniquement la base via Docker :
```bash
docker compose up -d postgres
```

#### B. Lancer le Backend (Rust)
```bash
cd backend
cargo run
```
Le modèle d'embedding NLP `MultilingualE5Small` est automatiquement téléchargé au premier lancement dans le cache utilisateur.

#### C. Lancer le Frontend (Astro)
Dans un autre terminal :
```bash
cd frontend
npm install
npm run dev
```

---

## 🧪 Tests Unitaires

Le backend inclut une suite de tests unitaires couvrant l'authentification (Argon2, JWT), la validation des DTOs et les calculs du moteur de recommandation (similarité cosinus, parsing, filtrage sémantique).

Pour exécuter les tests :
```bash
cd backend
cargo test
```

---

## 📑 Documentation de l'API

### Swagger UI
L'API intègre une documentation interactive OpenAPI générée automatiquement via Utoipa :
- Rendez-vous sur : [http://localhost:3000/swagger-ui](http://localhost:3000/swagger-ui)
- Cliquez sur **Authorize** pour insérer votre Bearer Token après connexion via `/api/auth/login`.

### Principaux Endpoints

| Méthode | Route | Description | Auth requise |
| :--- | :--- | :--- | :---: |
| `POST` | `/api/auth/register` | Création de compte utilisateur | Non |
| `POST` | `/api/auth/login` | Connexion et obtention du token JWT | Non |
| `GET` | `/api/users/me` | Profil de l'utilisateur connecté | ✅ |
| `GET` | `/api/users/{user_id}` | Profil public d'un utilisateur | Non |
| `GET` | `/api/books/search?q=...` | Recherche de livres via Google Books | Non |
| `GET` | `/api/books/{id}` | Détails complets d'un livre | Non |
| `GET` | `/api/books/{id}/ratings` | Moyenne et nombre de notes données | Non |
| `POST` | `/api/user-readings` | Marquer un livre comme lu et lui attribuer une note | ✅ |
| `GET` | `/api/users/{id}/readings` | Obtenir l'historique des lectures d'un utilisateur | Non |
| `DELETE`| `/api/user-readings/{book_id}`| Supprimer un livre de ses lectures | ✅ |
| `GET` | `/api/recommandations` | Obtenir des recommandations personnalisées (ML) | ✅ |
| `POST` | `/api/comments` | Ajouter un commentaire / avis sur un livre | ✅ |
| `GET` | `/api/comments/book/{book_id}`| Obtenir les commentaires et réponses d'un livre | Non |

---

## 🧠 Moteur de Recommandation par Machine Learning

Le moteur de recommandation repose sur une approche hybride de recherche sémantique :
1. **Extraction du profil utilisateur** : Sélection des meilleures lectures notées ($\ge 3$) par l'utilisateur connecté.
2. **Génération ciblée des candidats** : Récupération parallèle de livres similaires via l'API Google Books en ciblant les genres spécifiques et les auteurs aimés.
3. **Encodage vectoriel asymétrique** :
   - Le profil utilisateur est encodé avec le préfixe E5 `query: Titre: ... - Auteurs: ... - Genres: ... - Description: ...`.
   - Les livres candidats sont encodés avec le préfixe E5 `passage: Titre: ... - Auteurs: ... - Genres: ... - Description: ...`.
4. **Calcul du barycentre & Cosine Similarity** : Calcul de la similarité cosinus entre le vecteur moyen pondéré de l'utilisateur et chaque candidat, élimination des livres déjà lus ou doublons, et tri par score de pertinence décroissant.

