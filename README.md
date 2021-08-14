trunk serve<br>
elmのアーキテクチャを移植したフロントエンド用rustフレームワーク<br>
構造体を要素に持つベクタ(可変長配列)はクローンできない<br>
input参考 //https://crates.io/crates/seed/0.1.1<br>
Model -> 状態<br>
Msg -> イベント関数を列挙 引数の型も定義<br>
init関数 -> モデルの初期化<br>
update関数 -> イベント関数の具体的な処理内容<br>
view関数 -> 画面<br>
Node型 -> DOM APIに関するオブジェクトが継承するインターフェース<br>
クローントレイト -> 値を明示的コピー<br>
コピートレイト -> 値を暗黙的にコピー プリミティブ型は全てコピートレイトを実装している<br>
(let y = xをしてもxの値にアクセスできるのはこのため)<br>
unwrapメソッドはOkかErrを返す?<br>
```
/* 文字列をキーとし、(文字列キー: 文字列値)のハッシュマップを値として持つハッシュマップ(連想配列) */
HashMap<String, HashMap<String, String>>
```
unwrapはokなら結果 errならパニック<br>
collectはイテレータをベクタなどに変換するメソッド?<br>
ハッシュマップは連想配列(obj[key] = value) 初期値は設定できない<br>
辞書の使い方<br>
```
/* 変数型; 要素数 */
const select_pairs: [(&str, i32); 3] = [("低", 0), ("中", 1), ("高", 2)];
```