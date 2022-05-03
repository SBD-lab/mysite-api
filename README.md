# Rust Sample Web API with Azure Functions
## 開発環境
- Azure VM Linux (ubuntu 20.04)
- rustc 1.60.0 (7737e0b5c 2022-04-04)
- Azure Functions Core Tools (4.0.3971)
- Visual Studio Code (ローカル PC から SSH 接続)
## /api/inquiry
- お問い合わせフォーム用の Web API です。
- お問い合わせデータは Azure Cosmos DB (Mongo DB) に格納されます。
- 事前に Azure Cosmos DB (サーバレス) のアカウントとコレクションを作成しておきます。
- 以下の環境変数を設定しておきます。
    - `MONGODB_URL`: 作成した Mongo DB のプライマリ接続 URI
    - `MONGODB_DATABASE`: 作成した Mongo DB のデータベース名
    - `MONGODB_COLLECTION`: 作成した Mongo DB のコレクション名
- `rustup target add x86_64-unknown-linux-musl` でビルド対象を追加しておきます。
- `sudo apt install -y musl-tools` で musl のリンカーをインストールしておきます。
- `cargo build --release` でビルドします。
- `cp target/x86_64-unknown-linux-musl/release/mysite-api .` でビルドしたバイナリをプロジェクトルートディレクトリに配置します。
- `func start` でローカルデプロイします。
- Visual Studio Code の Azure Functions 拡張機能から関数アプリをデプロイします。
- Azure Functions の環境変数を設定して、関数アプリを再起動します。