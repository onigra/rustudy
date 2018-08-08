extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // 標準入力をString型で受け付ける
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 標準入力の改行をtrimして数値にparseする
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,     // parseに成功したらparseした値を返す
            Err(_) => continue, // 失敗したら次のループに進む
        };

        println!("You guessed: {}", guess);

        // secret_numberと標準入力から受け付けたguessを比較
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // 一致したら勝ちと判定し、ループを抜ける
            }
        }
    }
}
