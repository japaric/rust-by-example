fn main() {
    let n = 6is;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // This expression returns an `isize`
            10 * n
        } else {
            println!(", and is a big number, reduce by two");

            // This expression must return an `isize` as well
            n / 2
            // TODO ^ Try suppressing this expression with a semicolon
        };
    //   ^ Don't forget to put a semicolon here! All the `let` bindings need it

    println!("{} -> {}", n, big_n);
}
