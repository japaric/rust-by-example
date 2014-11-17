#![feature(macro_rules)]

macro_rules! assert_equal_len {
    ($a:ident, $b: ident, $func:ident, $op:tt) => {
        assert!($a.len() == $b.len(),
                "{}: dimension mismatch: {} {} {}",
                stringify!($func),
                ($a.len(),),
                stringify!($op),
                ($b.len(),));
    }
}

macro_rules! op {
    ($func:ident, $bound:ident, $op:tt, $method:ident) => {
        fn $func<T: $bound<T, T>>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op);

            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = x.$method(y);
            }
        }
    }
}

// implement add_assign, mul_assign, and sub_assign functions
op!(add_assign, Add, +=, add)
op!(mul_assign, Mul, *=, mul)
op!(sub_assign, Sub, -=, sub)

fn main() {
    let mut xs = Vec::from_elem(5, 0f64);
    let ys = Vec::from_elem(6, 1f64);

    // this operation will panic at runtime
    add_assign(&mut xs, &ys);
}

mod test {
    macro_rules! test {
        ($func: ident, $x:expr, $y:expr, $z:expr) => {
            #[test]
            fn $func() {
                for size in range(0u, 10) {
                    let mut x = Vec::from_elem(size, $x);
                    let y = Vec::from_elem(size, $y);
                    let z = Vec::from_elem(size, $z);

                    super::$func(&mut x, &y);

                    assert_eq!(x, z);
                }
            }
        }
    }

    // test add_assign, mul_assign and sub_assign
    test!(add_assign, 1u, 2u, 3u)
    test!(mul_assign, 2u, 3u, 6u)
    test!(sub_assign, 3u, 2u, 1u)
}
