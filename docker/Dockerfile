FROM rust:latest

LABEL Maintainer Yuhanun Citgez <y.citgez@student.utwente.nl>

# Postgres input
ARG POSTGRES_URL

# Set rustup to nightly
RUN rustup default nightly

# Make the directory.
RUN mkdir /data

# Clone the repo
RUN git clone https://github.com/Yuhanun/lipsum_best.git --recursive /data/lipsum_best && \
		cd /data/lipsum_best && \
		cargo build --release

# Set the Rocket.toml file
RUN echo "[global.databases] \n\
lipsum_best = { url = \"$POSTGRES_URL\" }" > /data/lipsum_best/Rocket.toml

# Set work directory.
WORKDIR /data/lipsum_best

# Run on docker launch
CMD cargo run --release
