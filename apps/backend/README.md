@@ -0,0 +1,42 @@
```
backend/
├── src/
│   ├── domain/
│   │   ├── entities/        # ビジネスエンティティ
│   │   ├── repositories/    # リポジトリのトレイト定義
│   │   ├── usecases/        # ユースケースのトレイト定義
│   │   └── value_objects/   # 値オブジェクト
│   │
│   ├── application/
│   │   ├── services/        # ユースケースの実装
│   │   ├── dtos/            # アプリケーション固有のデータ構造
│   │   └── errors/          # アプリケーションエラー
│   │
│   ├── infrastructure/
│   │   ├── database/
│   │   │   ├── models/      # sea-ormのエンティティモデル
│   │   │   ├── migrations/  # sea-ormのマイグレーション
│   │   │   └── repositories/ # sea-ormを使ったリポジトリ実装
│   │   └── config/          # 設定関連
│   │
│   ├── presentation/
│   │   ├── http/
│   │   │   ├── handlers/    # axumのハンドラー
│   │   │   ├── middlewares/ # axumのミドルウェア
│   │   │   └── routes/      # axumのルート定義
│   │   └── graphql/
│   │       ├── objects/      # GraphQLスキーマ定義
│   │       ├── resolvers/   # GraphQLリゾルバー
│   │       └── scalars/     # カスタムGraphQLスカラー型
│   │
│   └── main.rs             # アプリケーションのエントリーポイント
│
├── tests/                   # テスト
│   ├── domain/              # ドメインロジックのテスト
│   ├── application/         # アプリケーションロジックのテスト
│   ├── infrastructure/      # インフラストラクチャのテスト
│   └── presentation/        # プレゼンテーションのテスト
│
├── Cargo.toml
├── .env                     # 環境設定
└── migration                # データベースマイグレーション
    ├── src/
    │   ├── lib.rs
    │   ├── main.rs
    │   └── mYYYYMMDD_HHMMSS_xxx.rs
    └── Cargo.toml
```
