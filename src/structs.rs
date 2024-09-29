#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn create(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    fn area(&self) {
        println!("{}", self.width * self.height);
    }
    // fn area(self) {
    //     println!("{}", self.width * self.height);    // 所有権がarea関数に移動しているためエラー
    // }
}

pub fn run() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = user1;
    // println!("{}", user1.email); // 所有権がuser2に移動しているためエラー
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    println!("{:#?}", user1); //#[derive(Debug)]が必要
    let user2 = build_user(String::from("user2@xxx.com"), String::from("user2"));
    println!("{:#?}", user2);

    let rect = Rectangle::create(20, 20);
    println!("{:#?}", rect);
    rect.area();
    println!("{:#?}", rect); // 所有権がarea関数に移動しているためエラー
}
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
