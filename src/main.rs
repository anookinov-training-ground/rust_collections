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

    let mut s2 = String::from("foo");
    s2.push_str("bar");
    println!("s2 is {}", s2);

    let mut s3 = String::from("foo");
    let s4 = "bar";
    s3.push_str(s4);
    println!("s4 is {}", s4);

    let mut s5 = String::from("lo");
    s5.push('l');
    println!("s5 is {}", s5);

    let s6 = String::from("Hello, ");
    let s7 = String::from("world!");
    // + operator uses the add method that looks something like `fn add(self, s: &str) -> String {}`
    let s8 = s6 + &s7; // note s6 has been moved here and con no longer be used
    println!("s8 is {}", s8);

    let s9 = String::from("tic");
    let s10 = String::from("tac");
    let s11 = String::from("toe");
    let s12 = s9 + "-" + &s10 + "-" + &s11;
    println!("s12 is {}", s12);

    let s13 = String::from("tic");
    let s14 = String::from("tac");
    let s15 = String::from("toe");
    let s16 = format!("{}-{}-{}", s13, s14, s15);
    println!("s16 is {}", s16);

    // let s17 = String::from("hello");
    // let h = s17[0]; // error - Rust strings don't support indexing

    let hello = String::from("Hola");
    println!("{}", hello.len());

    let hello = String::from("Здравствуйте");
    println!("{}", hello.len());
    // let answer = &hello[0]; // error - Rust strings don't support indexing
    let answer = &hello[0..4];
    println!("{}", answer);
    // let answer = &hello[0..1];
    // println!("{}", answer); // panic as byte index 1 is not a char boundary

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for c in "สวัสดี".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    for b in "สวัสดี".bytes() {
        println!("{}", b);
    }

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:#?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("{}, {}", field_name, field_value); // field_name and field_value are invalid at this point

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:#?}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
