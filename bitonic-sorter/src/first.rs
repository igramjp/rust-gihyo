// pub はこの sort 関数が他のモジュールからアクセスできることを示す
// 引数 x の型 &mut [u32] について
//   & は値をポインタ経由で借用することを示す
//   mut は値が変更可能であることを示す
//   u32 型は 32 ビット符号なし整数
//   [u32] 型は u32 のスライス

pub fn sort(x: &mut [u32], up: bool) {
    // 未実装の意味．コンパイルは通るが実行すると panic する
    unimplemented!();
}

fn sub_sort(x: &mut [u32], up: bool) {
    unimplemented!();
}

fn compare_and_swap(x: &mut [u32], up: bool) {
    unimplemented!();
}
