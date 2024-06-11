####################################################################################################
## Builder
####################################################################################################
FROM rust:bullseye AS build

RUN apt-get update -y && \
    apt -y install libclang-dev

WORKDIR /rusty-trader
COPY ./ .
RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM gcr.io/distroless/cc-debian11

# Copy the binary from the build stage
COPY --from=build /rusty-trader/target/release/controller ./
COPY --from=build /rusty-trader/target/release/dash ./
COPY --from=build /rusty-trader/target/release/trader ./

ENTRYPOINT ["./controller"]