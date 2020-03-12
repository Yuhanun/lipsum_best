FROM rust:latest

LABEL Maintainer Yuhanun Citgez <y.citgez@student.utwente.nl>

# Set rustup to nightly
RUN rustup default nightly

# Make the directory.
RUN mkdir /data

# Clone the repo
RUN git clone https://github.com/Yuhanun/lipsum_best.git \
				--recursive /data/lipsum_best

# Get diesel_cli
RUN cargo install diesel_cli --no-default-features --features postgres

# Copy over startup.sh
COPY startup.sh /startup.sh

# Run on docker launch
CMD "/startup.sh"
