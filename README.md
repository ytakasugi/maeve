# maeve

---

## 参考

- [Rust の新しい HTTP サーバーのクレート Axum をフルに活用してサーバーサイドアプリケーション開発をしてみる](https://blog-dry.com/entry/2021/12/26/002649)
  - [stock-metrics](https://github.com/yuk1ty/stock-metrics)
- [Layered Architecture(Clean Architecture)の勘所](https://qiita.com/shunjikonishi/items/9cbf67314000cc42fbcc)
- [実装クリーンアーキテクチャ](https://qiita.com/nrslib/items/a5f902c4defc83bd46b8)

---

## リクエスト

- ユーザー作成
  - `curl -X POST -H "Content-Type: application/json" -d '{"user_name": "admin", "email": "admin@email.com", "password_hash": "PasswordAdmin", "user_role": "admin"}' http://localhost:8080/users`
- ユーザー取得
  - `curl http://localhost:8080/users/01GNV3TVA288YCRC57WZE183QA`
- カスタマー作成
  - `curl -X POST -H "Content-Type: application/json" -d '{"user_id": "01GPTXVCTG8D2CG4BYJYFR9HC3", "name": "admin", "zip_code": "100-0014", "address": "adminLocation", "phone": "999-9999-9999"}' http://localhost:8080/customers`
