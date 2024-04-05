# WbertoreRustLambdas
Source code for rust lambdas with common dependencies deployed to personal aws account.

## Starting a Local Server
Run the following command to start a local server:
`cargo lambda watch -p 9000`

The rust code will start up in a simulated lambda environment.

From here, you can make requests to the lambda, either with the cli:
```bash
curl -v -X POST \
  'http://localhost:9001/lambda-url/WbertoreRustLambdas/' \
  -H 'content-type: application/json' \
  -d '{ "command": "hi" }'
```

Or with a web-browser:
`http://localhost:9001/lambda-url/WbertoreRustLambdas/`

Right now the lambda is using the package name `WbertoreRustLambdas` in the Cargo.toml file. When more lambdas are
added, you can make requests by filling out the metadata section. See more info here:
https://www.cargo-lambda.info/commands/watch.html
