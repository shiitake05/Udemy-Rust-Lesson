pub fn run() {
    let s1 = String::from("hello");
    let s2 = s1; // 所有権がs1からs2に移動
                 // println!("{} {}", s1, s2); // 所有権がs2に移動しているためs1でエラー
    println!("{}", s2);

    let i1 = 1;
    let i2 = i1;
    println!("{} {}", i1, i2);
    println!("Stack address of i1: {:p}", &i1);
    println!("Stack address of i2: {:p}", &i2);

    let sl1 = "literal";
    let sl2 = sl1;
    println!("{} {}", sl1, sl2);
    println!("Stack address of sl1: {:p}", &sl1);
    println!("Stack address of sl2: {:p}", &sl2);

    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("Stacj address of s3: {:p}", &s3);
    println!("Stacj address of s4: {:p}", &s4);
    println!("Heap address of s3: {:p}", s3.as_ptr());
    println!("Heap address of s4: {:p}", s4.as_ptr());
    println!("{} {}", s3, s4);

    let s5 = String::from("hello");
    println!("Stack address of s5: {:p}", &s5);
    println!("Heap address of s5: {:p}", s5.as_ptr());
    println!("Len of s5: {}", s5.len());
    println!("Cap of s5: {}", s5.capacity());
    take_ownership(s5);
    // println!("{}", s5); // 所有権がtake_ownership関数に移動しているためエラー
    let s6 = String::from("hello"); // 所有権はs6に移動
    println!("Stack address of s6: {:p}", &s6);
    println!("Heap address of s6: {:p}", s6.as_ptr());
    println!("Len of s6: {}", s6.len());
    let s7 = take_giveback_ownership(s6); // 所有権はtake_giveback_ownership関数に移動し、その後s7に移動
    println!("Stack address of s7: {:p}", &s7);
    println!("Heap address of s7: {:p}", s7.as_ptr());
    println!("Len of s7: {}", s7.len());

    let s8 = String::from("hello");
    let len = calculate_length(&s8);
    println!("The length of '{}' is {}.", s8, len);

    let mut s9 = String::from("hello");
    change(&mut s9);
    println!("{}", s9);

    let s10 = String::from("hello");
    let r1 = &s10;
    let r2 = &s10;
    println!("{} {} {}", s10, r1, r2);

    let mut s10 = String::from("hello");
    let r1 = &s10;
    let r2 = &mut s10;
    // println!("{} {}", r1, r2); // 参照の可変性が異なるためエラー
    let mut s11 = String::from("hello");
    let r1 = &mut s11;
    // println!("{}", s11); // 所有権者であっても可変参照を取得しているためエラー
    // println!("{}", r1);
    println!("{}", r1);
    println!("{}", s11); // 参照を取得しているためエラーにならない

    let mut s12 = String::from("hello");
    let r1 = &s12;
    let r2 = &s12;
    println!("{} and {}", r1, r2);
    let r3 = &mut s12;
    *r3 = String::from("hello_updated");
    println!("{}", s12);
}
fn take_ownership(s: String) {
    println!("Stack address of s: {:p}", &s); // スタックアドレスが異なる
    println!("Heap address of s: {:p}", s.as_ptr()); // ヒープアドレスが同じ
    println!("Len of s: {}", s.len());
    println!("Cap of s: {}", s.capacity());
    println!("{}", s);
}
fn take_giveback_ownership(s: String) -> String {
    s
}
fn calculate_length(s: &String) -> usize {
    s.len()
}
fn change(s: &mut String) {
    s.push_str("_world");
}
