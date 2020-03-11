FROM rust:latest

LABEL Maintainer Roboteam <roboteamtwente@gmail.com>

# Set rustup to nightly
RUN rustup default nightly

# Make the directory.
RUN mkdir /data

# Clone the repo
RUN git clone https://github.com/Yuhanun/lipsum_best.git --recursive /data/lipsum_best

# Set work directory.
WORKDIR /data/lipsum_best

# Run on docker launch
CMD cargo run --release