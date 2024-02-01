####################################################################################################
## Builder
####################################################################################################
FROM rust:bullseye AS build

RUN apt-get update -y && \
    apt-get install -y ca-certificates

WORKDIR /rusty-trader
COPY ./ .
RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM gcr.io/distroless/cc-debian11

COPY --from=build /rusty-trader/target/release/rusty-trader ./
ENTRYPOINT ["./rusty-trader"]