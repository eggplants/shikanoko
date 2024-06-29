FROM rust:1-alpine3.19

# docker build -t eggplants/shikanoko
# docker run --rm eggplants/shikanoko

ARG VERSION
ENV VERSION=${VERSION:-master}

RUN cargo install --git https://github.com/eggplants/shikanoko --tag "${VERSION}"

ENTRYPOINT ["shikanoko"]
