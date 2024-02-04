# rusty-trader

[![Deploy Pipline](https://github.com/fubblea/rusty-trader/actions/workflows/DeployPipeline.yaml/badge.svg)](https://github.com/fubblea/rusty-trader/actions/workflows/DeployPipeline.yaml)

## Usage

Docker Compose:

```yaml
version: '3.3'
services:
  rusty-trader:
    container_name: rusty-trader
    build:
      context: ./
      dockerfile: Dockerfile
    ports:
      - 3000:3000
    environment:
      - ACCESS_KEY=${ALPACA_ACCESS_KEY}
      - SECRET_KEY=${ALPACA_SECRET_KEY}
```

Get Alpaca keys from here: <https://app.alpaca.markets/brokerage/new-account>
