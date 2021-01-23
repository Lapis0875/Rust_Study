# Chapter 02 : 추리 게임 만들어보기
추리 게임을 만들어보며, Rust 코드의 기본 구조를 파악해 보았습니다.

## 1. 프로젝트 파일, Cargo.toml
```toml
[package]
name = "guessing_game"
version = "0.1.0"
authors = ["Minjun Kim (Lapis0875) <lapis0875@teamif.io>"]

[dependencies]
```
Rust 프로젝트는 Cargo.toml 이라는 파일로 프로젝트를 관리합니다.

### `[package]`
package 오브젝트에는 프로젝트의 정보를 기입합니다.
`name`에는 프로젝트의 이름을, `version`에는 프로젝트의 버전을 작성합니다.
`authors`에는 저자들의 연락처를 배열로 기재합니다.

### `[dependencies]`
러스트에서는 코드의 패키지를 crate 라고 부릅니다.
dependencies 오브젝트에는 프로젝트의 의존성 정보 (프로젝트가 사용하는 crate 들) 를 기재합니다.
Java, Kotlin 에서 Maven, Gradle 등을 이용해 외장 라이브러리를 사용하는 것과 같은 패키지 의존성 기능을 Rust 는 Cargo 를 통해서 제공합니다.

## 2. 프로젝트 구조
```
src/
└ main.rs
Cargo.lock
Cargo.toml
```
Rust 프로젝트는 위와 같은 구조를 가집니다.
최상위 경로 (root) 에 Cargo.toml 파일이 위치합니다. 
Cargo.lock 파일은 Cargo 가 자동적으로 생성하게 되는 파일입니다.
src 경로 안에는 프로젝트의 코드가 위치합니다.

## 3. Rust 코드
```rust
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```
위 코드를 한 줄씩 알아보겠습니다.

### use 문
```rust
use std::io;
```
위 코드는 표준 라이브러리인 `std` 에서 `io` 라이브러리를 가져옵니다.
`std::io`는 사용자의 입력을 받는 것을 포함하여 io와 관련된 기능들을 제공합니다.

러스트는 모든 프로그램의 스코프에 [prelude](https://doc.rust-lang.org/std/prelude/index.html) 내의 타입들을 가져옵니다. 
만약 여러분이 원하는 타입이 prelude 내에 없다면, use 문을 활용하여 명시적으로 그 타입을 가져와야 합니다.

### main 함수
```rust
fn main() {...}
```
Rust 에서 함수를 정의할 때는 fn 키워드를 이용합니다.
```
fn 함수명(파라미터) -> 반환값 타입 {
    코드
}
```
함수는 위와 같은 구조를 가집니다.

### println!
```rust
println!("Guess the number! ~ TeamIF - RustStudy");
println!("Please input your guess :D");
```
`println!`은 string 을 화면에 표시하는 매크로입니다.

### let mut
```rust
let mut guess = String::new();
```
let 키워드는 변수를 생성합니다. Rust 에서 변수는 기본적으로 불변 (immutable) 입니다.
mut 키워드를 사용하면 가변 (mutable) 변수를 선언할 수 있습니다.
변수의 타입은 변수의 이름 뒤에 `: String` 처럼 타입을 표기하여 지정합니다.

생성된 변수에 `String::new()` 를 통해 새롭게 생성한 String 타입의 객체를 할당합니다.
`String` 타입은 표준 라이브러리에서 제공하는 확장 가능한(growable) UTF-8 인코딩의 문자열 타입입니다.
`::new` 에 있는 `::` 는 `new` 함수가 `String` 의 **연관함수** 임을 나타냅니다.
연관함수는 하나의 타입을 위한 함수이며, 이 경우에는 하나의 String 인스턴스가 아니라 String 타입을 위한 함수입니다.
몇몇 언어에서는 이것을 정적 메소드 라고 부릅니다. 
python 에서는 @classmethod, java 에서는 `static` 함수가 있네요.

`new` 함수는 새 객체를 생성하기 위한 함수로, 이 경우 빈 새 String 객체를 생성합니다.
`new` 함수는 새로운 값을 생성하기 위한 일반적인 이름 (생성자) 이므로 많은 타입에서 찾아볼 수 있습니다.

요약하자면 `let mut guess = String::new();` 라인은 새로운 빈 String 인스턴스와 연결된 가변변수를 생성합니다.

### io::stdin().read_line()
```rust
io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
```
만약 프로그램 시작점에 `use std::io` 가 없다면, 함수 호출 시 `std::io::stdin` 처럼 작성해야 합니다.
`stdin` 함수는 터미널의 표준 입력의 핸들(handle)의 타입인 [std::io::Stdin](https://doc.rust-lang.org/std/io/struct.Stdin.html) 의 인스턴스를 반환합니다.

코드의 다음 부분인 `.read_line(&mut guess)` 는 사용자로부터 입력을 받기 위해 표준 입력 핸들에서 `read_line` 메소드를 호출합니다.

`read_line`은 사용자가 표준 입력에 입력할 때마다 입력된 문자들을 하나의 문자열에 저장하므로 인자로 값을 저장할 문자열이 필요합니다.
그 문자열 인자는 사용자 입력을 추가하면서 변경되므로 가변이어야 합니다.
`&`는 코드의 여러 부분에서 데이터를 여러 번 메모리로 복사하지 않고 접근하기 위한 방법을 제공하는 참조자 임을 나타냅니다.
참조자는 복잡한 특성으로서 러스트의 큰 이점 중 하나가 참조자를 사용함으로써 얻는 안전성과 용이성입니다.
참조자는 변수처럼 기본적으로 불변임을 알기만 하면 됩니다.
하지만, guess 변수는 가변 변수이므로, 참조자를 가변으로 바꾸기 위해 `&guess` 가 아니라 `&mut guess` 를 사용합니다.

`.expect("Failed to read line");` 처럼 `.foo()` 형태의 문법으로 메소드를 호출할 경우, 긴 라인을 나누기 위해 다음 줄과 여백을 넣는 것이 바람직합니다.
위 코드를 아래처럼 쓸 수도 있습니다.
`io::stdin().read_line(&mut guess).expect("Failed to read line");`
하지만 하나의 긴 라인은 가독성이 떨어지므로, 두 개의 메소드 호출을 위한 라인으로 나누는 것이 좋습니다.

> Result 타입으로 잠재된 실패 다루기

이전에 언급한 것처럼 `read_line` 은 우리가 인자로 넘긴 문자열에 사용자가 입력을 저장할 뿐 아니라 하나의 값을 돌려 줍니다.
여기서 돌려준 값은 `io::Result` 입니다.
러스트는 표준 라이브러리에 여러 종류의 `Result` 타입을 가지고 있습니다.
대표적으로 제네릭 `Result` 이나 `io:Result` 가 있습니다.

`Result` 타입은 열거형(enumerations, `enums` 라고 부르기도 합니다.) 입니다.
열거형은 정해진 값들만을 가질 수 있으며, 이러한 값들은 열거형의 variants 라고 부릅니다.

`Result`의 `variants`는 `Ok`와 `Err` 입니다.
`Ok` 는 처리가 성공했음을 나타내며, 내부적으로 성공적으로 생성된 결과를 가지고 있습니다.
`Err` 는 처리가 실패했음을 나타내고, 그 이유에 대한 정보를 가지고 있습니다.

이러한 `Result` 는 에러 처리를 위한 정보를 표현하기 위해 사용됩니다.
`Result` 타입의 값들은 다른 타입들처럼 메소드들을 가지고 있습니다.
`io::Result` 인스턴스는 `expect` 메소드를 가지고 있습니다.
만약 `io::Result` 인스턴스가 `Err` 일 경우 `expect` 메소드는 프로그램이 작동을 멈추게 하고 `expect` 에 인자로 넘겼던 메세지를 출력하도록 합니다.
만약 `read_line` 메소드가 `Err` 를 돌려줬다면 그 에러는 운영체제로부터 생긴 에러일 경우가 많습니다.
만약 `io::Result` 가 `Ok` 값이라면 `expect` 는 `Ok` 가 가지고 있는 결과값을 돌려주어 사용할 수 있도록 합니다.
이 경우 결과값은 사용자가 표준 입력으로 입력했던 바이트의 개수입니다.

만약 `expect` 를 호출하지 않는다면 컴파일은 되지만 경고가 나타납니다.
```shell
$ cargo build
Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
warning: unused `std::result::Result` which must be used
--> src/main.rs:10:5
|
10 |     io::stdin().read_line(&mut guess);
|     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
|
= note: #[warn(unused_must_use)] on by default
```

러스트는 `read_line` 가 돌려주는 `Result` 값을 사용하지 않았음을 경고하며 일어날 수 있는 에러를 처리하지 않았음을 알려줍니다.
이 경고를 없애는 옳은 방법은 에러를 처리하는 코드를 작성하는 것이지만 만약 문제가 발생했을 때 프로그램이 멈추길 바란다면 `expect` 를 사용할 수 있습니다.

### println! - placeholder {}
`{}`는 변경자 (placeholder) 로써 값이 표시되는 위치를 나타냅니다.
`{}`를 이용하여 하나 이상의 값을 표시할 수도 있습니다.
첫번째 `{}`는 형식 문자열(format string) 이후의 첫번째 값을 표시하며, 두번째 {}는 두번째 값을 나타냅니다.
이후의 `{}`도 같은 방식으로 작동합니다.
다음 코드는 println!을 이용하여 여러 값을 표시하는 예제입니다.
```rust
let x = 5;
let y = 10;
println!("x = {} and y = {}", x, y);
```
이 코드는 `x = 5 and y = 10`을 출력합니다.

## 4. `rand` crate 추가하기
Ch02에 생성된 프로젝트는 실행이 가능한 binary crate 입니다.
`rand` crate 는 다른 프로그램에서 사용되기 위한 용도인 library crate 입니다.
```toml
[dependencies]
rand = "0.3.14"
```
`dependencies` 절은 프로젝트가 의존하고 있는 `crate` 들과 각각의 요구 버전을 Cargo 에 명시하는 곳입니다.
`dependencies` 절에 `rand = "0.3.14"` 를 추가합니다. 
이는 `rand` 크레이트의 유의적 버전인 `0.3.14` 를 명시하는 것입니다.
Cargo 는 버전을 명시하는 데 [유의적 버전 (Semantic Versioning)](https://semver.org/lang/ko/) 을 이용합니다.
`0.3.14` 는 `^0.3.14` 의 축약형이 되며 이는 버전 `0.3.14` 와 호환되는 API 를 제공하는 모든 버전임을 의미합니다.
