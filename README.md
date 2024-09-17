# vasko

Chores notification bot for Slack

[![vasko](https://img.shields.io/badge/ghcr.io-mathiazom%2Fvasko-blue?logo=docker)](https://github.com/users/mathiazom/packages/container/package/vasko)

### Quick Start

The example configuration file [`config.example.kdl`](config.example.kdl) demonstrates the basic setup for Slack integration, workers, reminders, tasks, and schedule. Copy it to your own `config.kdl` and tweak away.

```bash
cp config.example.kdl config.kdl
```

With a `config.kdl` in your working directory, you have at least two options for getting vasko off the ground:

#### Build and run from source
```bash
cargo build --release
```
```bash
./target/release/vasko
```
#### Pull and run prebuilt Docker image
```bash
docker pull ghcr.io/mathiazom/vasko:main
```
```bash
docker run -v $(pwd)/config.kdl:/app/config.kdl ghcr.io/mathiazom/vasko:main
```
#### ... or somewhere in the middle
```
docker build -t vasko -f docker/Dockerfile . 
```

---

<img alt="vasko portrait" width="512" style="width: 150px" src="vasko.jpg">

> I started my own little carpet and upholstery cleaning business. I've done it for 20 years. I live well.
>
> — Vasko da Gama, circa 2021 (probably)