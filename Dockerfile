FROM rust:1-bookworm as builder

RUN apt-get update && apt-get install -y \
    ca-certificates \
    dumb-init \
    libgtk-4-dev \
    librsvg2-dev

WORKDIR /app
COPY . .
RUN cargo build --bins --release

RUN ldd target/release/favicon-rs

# Hack to get shared libraries into the scratch image
RUN <<ENDRUN
    ldd target/release/favicon-rs | cut -f 3 -d ' ' | awk NF | grep -v "^[a-z]" | sort  >sharedlibs.txt

    cat sharedlibs.txt
    mkdir -p libs
    cp $(cat sharedlibs.txt) libs

ENDRUN

#CMD ["./target/release/favicon-rs"]

#FROM scratch
FROM debian:bookworm-slim
LABEL org.opencontainers.image.source https://github.com/FileFormatInfo/favicon-rs

ARG COMMIT="(not set)"
ARG LASTMOD="(not set)"
ENV COMMIT=$COMMIT
ENV LASTMOD=$LASTMOD

WORKDIR /app
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder /usr/bin/dumb-init /usr/bin/dumb-init
COPY --from=builder /app/target/release/favicon-rs /app/favicon-rs
COPY --from=builder /app/libs /lib/x86_64-linux-gnu
COPY ./static /app/static
ENTRYPOINT ["/usr/bin/dumb-init", "--"]
CMD ["/app/favicon-rs"]