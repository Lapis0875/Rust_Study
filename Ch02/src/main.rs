extern crate rand;

use std::io;    // 표준 라이브러리인 std 내부의 io 라이브러리를 가져옵니다.
use std::cmp::Ordering;
use rand::Rng;

/*
main 함수는 프로젝트의 진입점입니다.
*/
fn main() {
    println!("Guess the number! ~ TeamIF - RustStudy");     // println! 은 콘솔에 문자열 (string) 을 출력하는 매크로입니다.

    let secret_number: u8 = rand::thread_rng().gen_range(1, 101);

    // 무한루프입니다.
    loop {
        println!("Please input your guess :D");

        let mut guess: String = String::new();  // 사용자의 입력값을 저장할 변수를 생성합니다.

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line :(");      // 콘솔로부터 사용자의 입력을 받아, guess 변수에 저장합니다.

        /*
            guess 에 저장된 문자열을 숫자로 파싱합니다.
            parse() 는 Result 를 반환하므로, expect 메소드를 통해 문자열을 출력하고 종료해도 되지만
            match {} 구문을 이용해 값을 처리해도 됩니다.
            아래 match 구문은 Ok일 때, 파싱된 수 (num)을 guess 에 할당하고 (이 경우, guess 변수는 shadowing 됩니다.)
            Err 일 때 (_를 사용해서 모든 오류에 대해서 처리합니다), 수를 입력하라고 안내한 뒤 continue 문을 사용해 반복문의 시작으로 돌아갑니다.
        */
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number :(");
                continue;
            }
        };

        println!("You guessed {}!", guess);      // println! 의 placeholder 문법입니다.

        /*
            cmp 메소드를 사용해 두 객체를 비교합니다.
            비교한 결과는 Ordering 이라는 enums 로 반환되며, 이는 match 표현식을 이용해
            Ordering::Less, Ordering::Equal, Ordering::Greater 의 경우에 대해 처리할 수 있습니다.
        */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;  // break 문을 사용해 반복문을 종료합니다.
            },
            Ordering::Greater => println!("Too big!")
        }
    }
}