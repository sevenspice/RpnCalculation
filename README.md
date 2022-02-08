# 概要

参考書、[]()のPart2 実践のChapter 4を作成したもの。

逆ポーランド記法による計算処理を行うコマンドラインアプリケーション。

## 前提
| アプリケーション | バージョン               |
| :--------------- | :----------------------- |
| rustup           | `1.24.3`                 |
| rustc            | `1.57.0`                 |
| cargo            | `1.57.0`                 |

## 使い方
### ファイルから計算
```
cargo run -- -f ./sample.txt
```

### 標準入力から計算
```
cargo run -- console
```

## デバッグ
### 前提条件
| アプリケーション | バージョン |
| :--------------- | :----------------------- |
| lldb             | `>=10.0.0`               |

上記インストール済みであること。

### デバッグ実行例
```
rust-lldb target/debug/rpn_calculation -- -f ./sample.txt
```
上記コマンド実行後、対話型シェルに移る。

### デバッグコマンド例
LLDBの使い方は[Practical Debugging with LLDB](http://debugging-with-lldb.blogspot.com/)を参考にすると良い。

ここでは以下に簡単な例を示す。

#### ブレークポイントの設定例
```
breakpoint set --file rpn.rs --line 24
```

### デバッグの実行
```
run
```
* 実行バイナリが読み込まれていればデバッグが開始される

### 変数のウォッチ
ブレークポイントで停止した際にそのフレーム内の変数の中身を確認可能。
```
frame variable
```

### プログラムを進める
ステップイン
```
step
```
ステップオーバー
```
next
```

### デバッグを終了させる
```
exit
```
