# maeve

---

## 参考

- [Rust の新しい HTTP サーバーのクレート Axum をフルに活用してサーバーサイドアプリケーション開発をしてみる](https://blog-dry.com/entry/2021/12/26/002649)
  - [stock-metrics](https://github.com/yuk1ty/stock-metrics)

---

## リクエスト

- ユーザー作成
  - `curl -X POST -H "Content-Type: application/json" -d '{"user_name": "test03", "password_hash": "PasswordTest03", "user_role": "test"}' http://localhost:8080/users`
- ユーザー取得
  - `curl http://localhost:8080/users/01GNV3TVA288YCRC57WZE183QA`
- カスタマー作成
  - `curl -X POST -H "Content-Type: application/json" -d '{"name": "admin", "email": "admin@example.com", "address": "adminLocation", "phone": "999-9999-9999"}' http://localhost:8080/customers/01GNV3TVA288YCRC57WZE183QA`
