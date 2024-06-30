# shikanoko

[![Test](<https://github.com/eggplants/shikanoko/actions/workflows/test.yml/badge.svg>)](https://github.com/eggplants/shikanoko/actions/workflows/test.yml) [![Release Package](https://github.com/eggplants/shikanoko/actions/workflows/release.yml/badge.svg)](https://github.com/eggplants/shikanoko/actions/workflows/release.yml) [![Image Size](https://ghcr-badge.egpl.dev/eggplants/shikanoko/size?color=%2344cc11&tag=latest&label=image+size&trim=)](https://github.com/eggplants/shikanoko/pkgs/container/shikanoko)

## What

See: <https://x.com/ikawaha/status/1806852026661511505>

## Install

### Cargo

```bash
cargo install --git https://github.com/eggplants/shikanoko
```

### Binary

See: [Releases](https://github.com/eggplants/shikanoko/releases)

### Docker

```bash
docker pull ghcr.io/eggplants/shikanoko
```

## Run

### Cargo, binary

```shellsession
$ shikanoko
USAGE: shikanoko <positive int>

$ shikanoko 1
...しかのこのこのここしたんたん🦌🦌🦌
Loop duration: 205.09ms

$ shikanoko 2
...

$ shikanoko 3
...
```

### Docker

```shellsession
$ docker run --rm eggplants/shikanoko 1
...しかのこのこのここしたんたん🦌🦌🦌
Loop duration: 23.08ms
```
