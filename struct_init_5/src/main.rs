#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    signin_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        email: String::from("someone@example.com"),
        username: String::from("someone"),
        signin_count: 1,
    };
    println!("User1: {:?}", user1);

    println!("mutable struct");
    mut_struct();

    println!("new user");
    let u = new_user();
    println!("new user {:?}", u);

    println!("new user 1 - struct init short hand");
    let u = new_user_1(String::from("someone"), String::from("someone@example.org"));
    println!("new user {:?}", u);

    println!("struct_update_manual");
    struct_update_1();

    println!("struct_update_syntax");
    struct_update_sytax();

    println!("tuple structs");
    tuple_structs();

    println!("unit struct");
    unit_struct();
}

// Individual fields cannot be made mutable. The
// entire struct needs to be
fn mut_struct() {
    let mut user1 = User {
        active: true,
        email: String::from("someone@example.com"),
        username: String::from("someone"),
        signin_count: 1,
    };
    user1.signin_count = 2;
    println!("User1: {:?}", user1);
}

// new user with implicit return
fn new_user() -> User {
    User {
        active: false,
        email: String::from("anonymous@example.org"),
        username: String::from("anonymous"),
        signin_count: 0,
    }
}

// new user wtih username and email arguments
// (same name as struct members) using init short hand.
fn new_user_1(username: String, email: String) -> User {
    User {
        username,
        email,
        active: false,
        signin_count: 0,
    }
}

// update/create struct using other struct instance
// manual method
fn struct_update_1() {
    let u1 = User {
        username: String::from("someone"),
        email: String::from("someone@example.com"),
        active: false,
        signin_count: 0,
    };
    let u1a = User {
        username: String::clone(&u1.username),
        email: String::clone(&u1.email),
        active: true,
        signin_count: 1,
    };
    println!("User1 : {:?}", u1);
    println!("User1a: {:?}", u1a);
}

// struct update is like assignment (=), which means the values
// are moved. Used in cases where u1 is no longer useful. The values
// for `username` and `email` are moved. The other two are scalar types
// which implement the copy trait and hence stays on the stack.
fn struct_update_sytax() {
    let u1 = User {
        username: String::from("someone"),
        email: String::from("someone@example.com"),
        active: false,
        signin_count: 0,
    };

    let u2 = User {
        active: true,
        signin_count: 1,
        ..u1 // value moved
    };
    println!("User2: {:?}", u2);
}

// structs are similar to tuples with names associated to fields
// tuple structs are similar to tuples - the difference being
// each tuple struct type is different type and identified by a name.
fn tuple_structs() {
    struct Point(i32, i32, i32);
    struct Color(i32, i32, i32);

    let zero = Point(0, 0, 0);
    let black = Color(0, 0, 0);
    println!("point: x, y, z => {}, {}, {}", zero.0, zero.1, zero.2);
    println!("black: r, g, b => {}, {}, {}", black.0, black.1, black.2);
}

// unit like struct
// just like a tuple unit. Unit like struct does not have fields.
fn unit_struct() {
    struct AlwaysEqual;
    let _result = AlwaysEqual;
    // this can be used to implement traits with no values.
}
