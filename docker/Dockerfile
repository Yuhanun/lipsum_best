FROM rust:latest

LABEL Maintainer Yuhanun Citgez <y.citgez@student.utwente.nl>

# Postgres input
ARG POSTGRES_URL

# Set rustup to nightly
RUN rustup default nightly

# Make the directory.
RUN mkdir /data

# Clone the repo
RUN git clone https://github.com/Yuhanun/lipsum_best.git \
				--recursive /data/lipsum_best

# Set the Rocket.toml file
RUN echo "[global.databases] \n\
waaier = { url = \"$POSTGRES_URL\" }" > /data/lipsum_best/Rocket.toml

# Some minor diesel setup stuff
RUN echo "DATABASE_URL=$POSTGRES_URL" > .env

# Get diesel_cli
RUN cargo install diesel_cli

# Setup diesel
RUN cd /data/lipsum_best && \
		diesel setup && \
		diesel migration run --database-url="$POSTGRES_URL"

# Set work directory.
WORKDIR /data/lipsum_best

# Run on docker launch
CMD git pull && cargo run --release
