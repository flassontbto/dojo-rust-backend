# Rust pour le backend

Anatomie d'un serveur écrit en Rust.

## Initialisation du projet

Quand il s'agit de Rust, `cargo` reste le principal outil
pour gérer ses dépendances :

```sh
cargo create --bin rust-backend-dojo
```

## Diesel : l'ORM qui carbure

[Diesel](http://diesel.rs/) s'appuie autant que possible sur le typage permis par Rust
pour prévenir les erreurs à la compilation plutôt qu'à l'exécution.

### Setup

Comme toute dépendance Rust, l'ajout se fait via le fichier [Cargo.toml](./Cargo.toml) :

```toml
[dependencies]
diesel = { version = "1.4.5", features = ["sqlite"], default-features = false }
dotenv = "0.15.0"
```

Diesel fournit un CLI à installer à part du projet pour faciliter les manipulations.
Par défaut, le support PostgreSQL est inclus, mais seul SQLite nous intéresse pour le dojo :

```sh
cargo install diesel-cli --no-default-features --features sqlite
```

Le CLI gère la création et l'initialisation de la base de données,
donc il peut récupérer l'accès via la variable d'environnement `DATABASE_URL`.
Celle-ci peut aussi être renseigné dans un [.env](./.env) à la racine du projet.
Pour initialiser la base :

```sh
diesel setup
```

Cette commande crée également un [dossier de migration](./migrations).
Notre base est un peu vide pour le moment, il est temps de changer ça.

```sh
diesel migration create baseline # <- Nom arbitraire

> Creating migrations/2021-01-08-145610_baseline/up.sql
> Creating migrations/2021-01-08-145610_baseline/down.sql
```

Les deux fichiers créés `up.sql` et `down.sql` sont
à compléter à la main, et correspondent respectivement à
la montée et descente de migration. Sans automatisme,
Diesel prend le parti de laisser l'utilisateur écrire les
migrations adaptées au SGBD utilisé.

Une fois prête, la migration peut être exécutée :

```sh
diesel migration run
```
Cela génère un [fichier source de liaison](./src/schema.rs).
Pour tester la descente de version, il est possible de
rejouer la migration :


```sh
diesel migration redo
```

## Rocket : un serveur web tout compris

Rocket joue la carte du "tout-en-un",
en proposant une intégration avec les outils
les plus courants de l'écosystème Rust.
Sérialisation, base de données, cookies, environnements, tout est déjà prévu.

ℹ️ D'autres implémentations de serveur web existent,
avec des promesses et des compromis différents.
On peut citer [actix](https://actix.rs), [warp](https://github.com/seanmonstar/warp) ou [tide](https://github.com/http-rs/tide), mais Rocket constitue un très bon équilibre
entre performance et courbe d'apprentissage.

### Setup

**Note** Nous utiliseront la version `0.5` de Rocket,
qui introduit le support de `async/await`.
Celle-ci n'est pas encore publiée sur le registry car certaines
fonctionnalités plus avancées que l'usage de ce code
sont en cours d'implémentation.

Comme toujours, la dépendance va dans le fichier [Cargo.toml](Cargo.toml) :

```toml
[dependencies]
rocket = { git = "https://github.com/SergioBenitez/Rocket.git" }
```

## Diesel + Rocket : fusion !

Rocket propose une crate `rocket_contrib` pour
les intégrations à des crates externes :

-   Bases de données (diesel)
-   Sérialisation (serde)
-   Templating (mustache/tera)

### Setup (avec Diesel)

```toml
[dependencies]
rocket_contrib = { features = ["diesel_sqlite_pool"], default-features = false, git = "https://github.com/SergioBenitez/Rocket.git" }
```

La configuration de la base de données passe par un fichier [Rocket.toml](Rocket.toml) :

```toml
[global.databases.db]
url = "./database.sqlite"
```

### Le pooling

L'intégration diesel supporte automatiquement le pooling
des connexions à l'aide de la crate `r2d2`. On utilise pour
cela le système de _fairing_ de Rocket, similaire aux
_middlewares_ d'autres frameworks.
