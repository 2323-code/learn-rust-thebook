// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
// 定数には代入できない（代入しようとするとエラーになる）

fn main() {
    // let x = "hello";
    // println!("The value of x is: {x}");
    // let mut x = 6;
    // println!("The value of x is: {x}");
    // // 6が入ってる
    // x = 8;
    // println!("The value of x is: {x}");
    // // 8が入ってる（4行目のxが使われる）
    // // → immutable大事！
    // println!("The value of x is: {THREE_HOURS_IN_SECONDS}");
    // // THREE_HOURS_IN_SECONDS = 1;
    // // println!("The value of x is: {THREE_HOURS_IN_SECONDS}");
    // // invalid left-hand side of assignment

    // // シャドウ
    // let x = 5;
    // let x = x + 1;
    // println!("x is {x}");
    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is {x}");
    // }
    // println!("The value of x is {x}");

    // let guess: u32 = "42".parse().expect("Not a number!");
    // // 42はstrじゃないとエラーになる(parse使えない)
    // println!("hey {guess}");
    // let char = 'Z';
    // let str = "Z";
    let tup = (500, 6.4, 1, "hello");
    // 型ごちゃ混ぜでも通る〜〜！！！
    // let (x, y, z) = tup;
    // println!("y is {y}");
    println!("{}", tup.3);
    // let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    // println!("{}", a[0]);
    // 配列は全て同じ型じゃないとエラーになる！
    // another_func();
    // let x = five();
    // println!("The value is {}", x);
    //
    // let condition = true;
    // let number: char = if condition { '5' } else { '6' };
    // trueだった場合の値の方にfalseの結果の型は縛られる
    // よりシンプルに自然にかけていいな！
    // println!("the value of number is {}", number);
    for number in (1..4).rev() {
        println!("{}", number);
    }
    println!("LIFTOFF!!")
}

fn another_func() {
    println!("Another function");
}

fn five() -> i32 {
    5
    // 5にセミコロンをつけると()が帰ってくる
    // ()は何もしない、という意味
    // 参考: https://doc.rust-jp.rs/book-ja/ch03-03-how-functions-work.html#:~:text=%E3%81%93%E3%81%AE%E3%81%93%E3%81%A8%E3%81%AF%E3%80%81%20()%E3%80%81%E3%81%A4%E3%81%BE%E3%82%8A%E7%A9%BA%E3%81%AE%E3%82%BF%E3%83%97%E3%83%AB%E3%81%A8%E3%81%97%E3%81%A6%E8%A1%A8%E7%8F%BE%E3%81%95%E3%82%8C%E3%81%A6%E3%81%84%E3%81%BE%E3%81%99%E3%80%82%E3%81%9D%E3%82%8C%E3%82%86%E3%81%88%E3%81%AB%E3%80%81%E4%BD%95%E3%82%82%E6%88%BB%E3%82%8A%E5%80%A4%E3%81%8C%E3%81%AA%E3%81%8F%E3%80%81%E3%81%93%E3%82%8C%E3%81%8C%E9%96%A2%E6%95%B0%E5%AE%9A%E7%BE%A9%E3%81%A8%E7%9F%9B%E7%9B%BE%E3%81%99%E3%82%8B%E3%81%AE%E3%81%A7%E3%80%81%20%E7%B5%90%E6%9E%9C%E3%81%A8%E3%81%97%E3%81%A6%E3%82%A8%E3%83%A9%E3%83%BC%E3%81%AB%E3%81%AA%E3%82%8B%E3%82%8F%E3%81%91%E3%81%A7%E3%81%99%E3%80%82
}
