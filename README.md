# sam-lambda-rust

AWS SAMでRustでのLambda関数をデプロイします。

## 初期ディレクトリ構成

まずは下記のような構成を目指します。`samconfig.toml` は、初回デプロイ時に生成します。

```txt
sam-lambda-rust
  ├── src
  │   └── main.rs
  ├── Cargo.toml
  ├── template.yaml
  └── Makefile
```

## 手順

1. `$ sam build`
2. `$ $ echo '{"body":"{\"data\": \"hello\"}"}' | sam local invoke --event -`
3. `$ sam deploy --guided`
   - 2回目以降は、`$ sam deploy` 