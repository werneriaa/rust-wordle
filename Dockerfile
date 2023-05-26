FROM rust:latest
WORKDIR /wordle
COPY wordle /wordle/
RUN cargo build
ENTRYPOINT [ "cargo", "run", "--" ]
CMD ["echo", "no start up commands"]