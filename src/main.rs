fn main() {
    let v: Vec<i32> = Vec::new();
    println!("{:#?}", v);
    let v = vec![1, 2, 3];
    println!("{:#?}", v);

    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);
    println!("{:#?}", v2);

    {
        let v3 = vec![1, 2, 3, 4];
        println!("{:#?}", v3);
    } // <- v3 goes out of scope and is freed here

    let v4 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v4[2];
    println!("The third element is {}", third);
    match v4.get(2)     {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v5 = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v5[100]; // panic
    // println!("{:#?}", does_not_exist);
    let does_not_exist = v5.get(100); // not panic but return None
    println!("{:#?}", does_not_exist);

    let mut v6 = vec![1, 2, 3, 4, 5];
    // let first = &v6[0];
    v6.push(6); // cannot borrow `v6` as mutable because it is already borrowed as immutable
    // println!("The first element is: {}", first);
    println!("{:#?}", v6);

    let v7 = vec![100, 32, 57];
    for i in &v7 {
        println!("{}", i);
    }

    let mut v8 = vec![100, 32, 57];
    for i in &mut v8 {
        *i += 50;
        println!("{}", i);
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:#?}", row);

    let mut s = String::new();
    println!("{}", s);
    s = "Hello world!".to_string();
    println!("{}", s);

    let data = "initial contents";
    let s = data.to_string();
    println!("{}", s);
    let s = "initial contents".to_string(); // the method also works on a literal directly
    println!("{}", s);
    let s = String::from("initial contents");
    println!("{}", s);

    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");
    let hello = String::from("สวัสดี");
    println!("{}", hello);
}
