/// KIBなどの単位を扱います



#[macro_export]
/// MB -> KIBに変換します
/// マクロにしているのはCONST定数や配列のサイズなどに対応するためです。
macro_rules! kib_from_mb {
    ( $x:expr ) => {
        {
           $x * 1024
        }
    };
}


