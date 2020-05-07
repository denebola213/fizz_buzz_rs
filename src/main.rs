fn main() {
    // fizz_buzz1()
    // fizz_buzz2()
    // fizz_buzz3()
    // fizz_buzz4()
    // fizz_buzz5()
    // fizz_buzz6()
    fizz_buzz7()
}

fn fizz_buzz1() {
    let mut x = 1;

    while x <= 100 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x);
        }
        x += 1;
    }
}

fn fizz_buzz2() {
    for x in 1 .. 101 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x);
        }
    }
}

fn fizz_buzz3() {
    for x in 1 .. 101 {
        match x % 15 {
            0 => println!("FizzBuzz"),
            3 | 6 | 9 | 12 => println!("Fizz"),
            5 | 10 => println!("Buzz"),
            _ => println!("{}", x),
        }
    }
}

fn fizz_buzz4() {
    for x in 1 .. 101 {
        match x % 15 {
            e if e % 15 == 0 => println!("FizzBuzz"),
            e if e % 3 == 0 => println!("Fizz"),
            e if e % 5 == 0 => println!("Buzz"),
            _e => println!("{}", x),
        }
    }
}

fn fizz_buzz5() {
    for x in 1 .. 101 {
        match (x % 3, x % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", x),
        }
    }
}

fn fizz_buzz6() {
    for x in 1 ..= 100 {
        let s = match (x % 3, x % 5) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            _      => x.to_string(),
        };
        println!("{}", s)
    }
}

fn fizz_buzz7() {
    for x in 1 ..= 100 {
        let tmp; // 値のスコープを広げるための変数
        let s = match (x % 3, x % 5) {
            (0, 0) => "FizzBuzz",
            (0, _) => "Fizz",
            (_, 0) => "Buzz",
            _      => {tmp = x.to_string(); &tmp}, // `tmp`を`x.to_to_string()`に束縛して参照をとる
        };
        println!("{}", s);
    }
}
