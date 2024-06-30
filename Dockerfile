FROM debian:stable-slim as builder

# docker build -t eggplants/shikanoko
# docker run --rm eggplants/shikanoko

ARG VERSION
ENV VERSION ${VERSION}

WORKDIR /work
RUN <<-EOF
    set -x
    apt-get update
    apt-get install -y --no-install-recommends ca-certificates unzip wget
    rm -rf /var/lib/apt/lists/*

    wget "https://github.com/eggplants/shikanoko/releases/download/${VERSION}/shikanoko_${VERSION}_x86_64-unknown-linux-musl.zip" -O shikanoko.zip
    unzip shikanoko.zip shikanoko
    mv shikanoko /shikanoko
EOF

FROM gcr.io/distroless/static

COPY --from=builder /shikanoko /

ENTRYPOINT ["/shikanoko"]
