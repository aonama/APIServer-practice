# APIServer-practice
APIサーバー構築の練習用

## 言語
Rust

## webフレームワーク
actix-web

## 使い方
1. リポジトリをcloneし、vscodeで開く
1. Reopen in Container
1. コンテナ内で/home/backendに移動し、``$ cargo run``
1. curlやブラウザ等でHTTPリクエストを飛ばす

## 実装済み機能
GETリクエスト  
``/``Hello World!を返す  
``/hey``Hey there!を返す  
``/get_message``log.txtの内容を返す  

POSTリクエスト  
``/echo``送信した文字列をそのまま返す  
``/send_message``送信した文字列をlog.txtに保存する
