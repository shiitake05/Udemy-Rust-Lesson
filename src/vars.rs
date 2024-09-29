pub mod sub_a;
pub mod sub_b;

const MAX_POINTS: u32 = 100_000; // 定数は大文字で定義する
                                 // constは関数内以外でも使用可能
                                 // letは関数内でのみ使用可能

pub fn run() {
    println!("Here is vars module!!");
    // sub_a::func_a();
    // sub_b::func_b();

    // let x = 5;
    let mut x = 5; // let mutで変数を変更可能にする
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let _i1 = 3; // 「_」で意図的に使用していないことを明示する
    let _f1 = 0.1;
    println!("{}", usize::BITS); // システムのサイズを表示
    println!("Memory address of const is: {:p}", &MAX_POINTS); // MAX_POINTSのメモリアドレスを表示

    let i2: i64 = 1;
    let i3: i64 = 2;
    println!("Stack address of i2 is: {:p}", &i2); // スタックのアドレスを表示
    println!("Stack address of i3 is: {:p}", &i3); // 最後の数字が8バイト分ずれている

    let y = 5;
    println!("Stack address of y is: {:p}", &y);
    let y = y + 1;
    println!("Stack address of y is: {:p}", &y);
    let y = y * 2;
    println!("Stack address of y is: {:p}", &y); // yの変数の値は新しい領域に格納されている
    println!("The value of y is: {}", y); // 12
    {
        let y = 0;
        println!("The value of y is: {}", y); // 0
    }
    println!("The value of y is: {}", y); // 12

    let t1 = (500, 6.4, "dummy"); // Rustのタプル
    let (_x, _y, _z) = t1; // タプルの分解の一方法
    println!("The value of t1 is: {} {} {}", t1.0, t1.1, t1.2); // タプルの分解の一方法

    let mut t2 = ((0, 1), (2, 3));
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2; // 参照を使ったタプルの分解
    *x1_ptr = 5; // 参照外しを使って値を変更
    *y1_ptr = -5; // 参照外しを使って値を変更
    println!("{:?}", t2);

    let a1 = [1, 2, 3, 4, 5]; // 配列
    let a2 = [0; 10]; // 配列の初期化
    println!("{:?} {:?} {} {}", a1, a2, a1[2], a1[3]);

    // 文字列スライスとString型
    let s1 = "helloこんにちは挨拶"; // 26bytes（日本語は3bytes）
    let s2 = "hello"; // 5bytes
    println!("Stack address of s1 is: {:p}", &s1);
    println!("Stack address of s2 is: {:p}", &s2); // 16bytes分ずれている
    println!("Static memory address of s1 is: {:?}", s1.as_ptr()); // 静的領域の先頭アドレスの場所を表示
    println!("Static memory address of s2 is: {:?}", s2.as_ptr());
    println!("Len of s1 is: {}", s1.len()); // 文字列の長さを表示
    println!("Len of s2 is: {}", s2.len());

    let mut s1 = String::from("hello");
    let mut s2 = String::from("helloworld");
    println!("Stack address of s1 is: {:p}", &s1); // 24bytes分ずれている
    println!("Stack address of s2 is: {:p}", &s2);
    println!("Heap memory address of s1 is: {:?}", s1.as_ptr()); // ヒープ領域の先頭アドレスの場所を表示
    println!("Heap memory address of s2 is: {:?}", s2.as_ptr());
    println!("Len of s1 is: {}", s1.len()); // 文字列の長さを表示
    println!("Len of s2 is: {}", s2.len());
    println!("Capacity of s1 is: {}", s1.capacity()); // 文字列の容量を表示
    println!("Capacity of s2 is: {}", s2.capacity());
    s1.push_str("_new1"); // 文字列の追加
    s2.push_str("_new2");
    println!("{} {}", s1, s2);
}
