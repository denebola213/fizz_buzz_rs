fn main() {
    // fizz_buzz1()
    // fizz_buzz2()
    // fizz_buzz3()
    // fizz_buzz4()
    // fizz_buzz5()
    // fizz_buzz6()
    // fizz_buzz7()
    // fizz_buzz10();
    // fizz_buzz11();
    fizz_buzz12();
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

// 8-9 day skipped

fn fizz_buzz10() {
    fn fz(x: i32) -> String {
        match (x % 3, x % 5) {
        (0, 0) => format!("FizzBuzz"),
        (0, _) => format!("Fizz"),
        (_, 0) => format!("Buzz"),
            _  => x.to_string(),
        }
    };

    let res =  (1 ..= 100).map(fz).collect::<Vec<_>>().join("\n");

    println!("{}", res);
}

fn fizz_buzz11() {
    fn fz<T>(x: T, div_a: T, div_b: T, zero: T) -> String
    where T: Rem<T, Output=T> + Eq + Copy + ToString {
        match (x % div_a == zero , x % div_b == zero) {
            (true, true) => format!("FizzBuzz"),
            (true, _) => format!("Fizz"),
            (_, true) => format!("Buzz"),
                _  => x.to_string(),
        }
    };

    (1 ..= 100).map(|x: u32| fz(x, 3,5, 0)).for_each(|x| println!("{}", x));
}

struct FizzBuzz<T> {
    div_a: T,
    div_b: T,
    zero: T,
}

impl<T> FizzBuzz<T> {
    fn new(div_a: T, div_b: T, zero: T) -> Self {
        FizzBuzz {div_a, div_b, zero}
    }
}

trait ToFzStr<T> {
    fn to_str(&self, x: T) -> String;
}

use std::ops::Rem;
fn common_fz_str<T>(x: T, div_a: T, div_b: T, zero: T) -> String
where T: Rem<T, Output=T> + Eq + Copy + ToString {
    match (x % div_a == zero , x % div_b == zero) {
        (true, true) => format!("FizzBuzz"),
        (true, _) => format!("Fizz"),
        (_, true) => format!("Buzz"),
            _  => x.to_string(),
    }
}

impl ToFzStr<i32> for FizzBuzz<i32> {
    fn to_str(&self, x: i32) -> String {
        common_fz_str(x, self.div_a, self.div_b, self.zero)
    }
}

fn fizz_buzz12() {
    (1 ..= 100).map(|x| FizzBuzz::new(3, 5, 0).to_str(x)).for_each(|x| println!("{}", x))
}

