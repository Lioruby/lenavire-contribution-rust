FROM rust:1.85.0 as builder

# Installer les dépendances nécessaires pour la compilation
RUN apt-get update && \
  apt-get install -y pkg-config libssl-dev libpq-dev && \
  apt-get clean && \
  rm -rf /var/lib/apt/lists/*

# Mettre à jour Rust vers la dernière version
RUN rustup update stable

# Installer diesel_cli pendant la phase de build
RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /usr/src/app

# Copier d'abord juste les fichiers nécessaires pour la dépendance
COPY Cargo.toml .
# Créer un fichier src/main.rs vide pour permettre la génération du Cargo.lock
RUN mkdir src && \
  echo "fn main() {}" > src/main.rs && \
  cargo build --release && \
  rm -rf src

# Maintenant copier le vrai code source
COPY . .
# Et rebuild avec le vrai code
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && \
  apt-get install -y libpq5 && \
  apt-get clean && \
  rm -rf /var/lib/apt/lists/*

# Créer et définir le répertoire de travail de l'application
WORKDIR /app

# Copier les fichiers nécessaires
COPY --from=builder /usr/src/app/target/release/lenavire-contribution-rust ./
COPY --from=builder /usr/local/cargo/bin/diesel ./
COPY --from=builder /usr/src/app/migrations ./migrations
COPY --from=builder /usr/src/app/.env ./.env

EXPOSE 8080

# Exécuter les migrations puis démarrer l'application
CMD ["sh", "-c", "./diesel migration run && ./lenavire-contribution-rust"]
