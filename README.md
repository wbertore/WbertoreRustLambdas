# WbertoreRustLambdas
Source code for rust lambdas with common dependencies deployed to personal aws account.

## Local development Pre-requisites
You need the following pre-reqs to build and run this package:
1. rustup: `brew install rustup-init`
2. rust, with target to al2 arm64: `rustup-init --target aarch64-unknown-linux-gnu`
3. cargo-lambda:
   i `brew tap cargo-lambda/cargo-lambda`
   ii. `brew install cargo-lambda`

## Building locally
Run the following command
```bash
cargo lambda build --release --arm64 --output-format zip
```

## Starting a Local Server
Run the following command to start a local server:
`cargo lambda watch -p 9000`

The rust code will start up in a simulated lambda environment.

From here, you can make requests to the lambda, either with the cli:
```bash
curl -v -X POST \
  'http://localhost:9000/lambda-url/WbertoreRustLambdas/' \
  -H 'content-type: application/json' \
  -d '{ "command": "hi" }'
```

Or with a web-browser:
`http://localhost:9000/lambda-url/WbertoreRustLambdas/`

Right now the lambda is using the package name `WbertoreRustLambdas` in the Cargo.toml file. When more lambdas are
added, you can make requests by filling out the metadata section. See more info here:
https://www.cargo-lambda.info/commands/watch.html
