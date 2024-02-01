####################################################################################################
## Builder
####################################################################################################
FROM rust:bullseye AS build

WORKDIR /rusty-trader
COPY ./ .
RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM debian:bullseye-slim
RUN apt-get update -y && \
    apt-get install -y ca-certificates
COPY --from=build /rusty-trader/target/release/rusty-trader ./
ENTRYPOINT ["./rusty-trader"]