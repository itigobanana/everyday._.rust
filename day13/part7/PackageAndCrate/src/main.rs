crate...コンパイルの最小単位
->binary crate...main関数を持つ
->library crate...他のプログラムから呼び出されて使われる部品
パッケージはクレートをまとめる箱
->パッケージはいくらでもbinary crateを持てるが、library crate
は最大一つまで。パッケージはこれらのどちらか最低一つを持たないといけない

cargo newでファイルの名前と置き場所を見るだけで勝手に中身を判断してくれる
->src/main.rsがあったら...これはbinary crate！
-> src/lib.rsがあったら...これはlibrary crate!
->両方あったら...どっちもある！
src/bin/があったら...binに入るファイルは別々のbinary crate!