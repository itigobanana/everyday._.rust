一つの巨大なファイルを複数のファイルにどうやってきれいに切り分けるか？？

1️⃣modは目次になる

mod front_of_house{
    pub mod hositing{
        pub fn add_to_waitlistz(){}
    }
}

で中身を全部書くと長い、、

mod front_of_house;

;で終了すると、コンパイラは自動的にsrc/front_of_house.rsを探してくれる

2️⃣孫モジュールはフォルダを使って整理する

3️⃣pythonと違って、rustのmodは、モジュールツリーの構造を定義する目次