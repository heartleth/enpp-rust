FROM rust

WORKDIR /
COPY . /enpp-rust
WORKDIR /enpp-rust
RUN cargo build --release

CMD ["/enpp-rust/target/release/engpp"]