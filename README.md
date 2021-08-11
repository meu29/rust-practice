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