コードが長くなってきたら、modというキーワードを使ってグループを作る。
⭐️プログラムの入り口は必ずsrc/main.rsかsrc/lib.rs
⭐️全てをmain.rsにおくと不便...main.rs内に[mod garden;]と書くと、
コンパイラは別ファイルがあると理解し、src/garden.rsというファイルを探して読み込む
⭐️別のモジュールにある機能を使いたいときは[::]を使って住所を書く
例：crate:garden:vegetables:Asparagus
⭐️rustのモジュールは基本非公開。他のモジュールからも使えるように公開したいときはpubを先頭につける
例：pub mod garden;
⭐️長い住所を書くのがめんどくさい時（さっきのAsparagusとか）は、ファイルの先頭で
use crate::garden::vegetables::Asparagus;と宣言しておくと、以降そのファイル内では
Asparagusと書くだけで使える！