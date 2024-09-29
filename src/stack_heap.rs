enum List {
    Node(i32, Box<List>), // 4バイトのi32と8バイトのListへのポインタ
    // Listと表記すると無限ループになってしまう
    Nil, // 0バイト
}

pub fn run() {
    // let a1: [i8; 7000000] = [1; 7000000];
    // let a1: [i8; 9000000] = [1; 9000000]; // スタックオーバーフローが発生する

    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];
    let mut v3 = vec![9, 10];
    println!("Stack address of v1 is: {:p}", &v1); // 24byte
    println!("Stack address of v2 is: {:p}", &v2);
    println!("Heap memory address of v1 is: {:p}", v1.as_ptr());
    println!("Len of v1 is: {}", v1.len());
    println!("Capacity of v1 is: {}", v1.capacity());
    v1.insert(1, 10); // 要素を挿入
    println!("{:?}", v1);
    v1.remove(0); // 要素を削除
    println!("{:?}", v1);
    v1.append(&mut v3); // v3をv1に追加
    println!("{:?}", v1);
    println!("{:?}", v3);

    let t1: (i64, String) = (10, String::from("hello"));
    println!("Stack address of tuple data is: {:p}", &t1); // スタックの先頭アドレス
    println!("Heap memory address of t1.1: {:?}", t1.1.as_ptr()); // ヒープの先頭アドレス
    println!("Len of t1.1: {}", t1.1.len());
    println!("Capacity of t1.1: {}", t1.1.capacity());
    let mut b1 = Box::new(t1);
    (*b1).1 += "world"; // 参照外しでBoxの中身を変更
    println!("{} {}", b1.0, b1.1);
    println!("Stack address of box data is: {:p}", &b1);
    println!("Heap address of tuple data is: {:p}", b1);
}
