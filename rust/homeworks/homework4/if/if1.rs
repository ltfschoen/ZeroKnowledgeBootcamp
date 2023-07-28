// if1.rs

// use std::cmp::max;

pub fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
    // Do not use:
    // - another function call
    // - additional variables
    // Execute `zustlings hint if1` for hints

    if a > b {
        return a;
    } else if b > a {
        return b;
    } else {
        panic!();
    }
    // doesn't work for some reason since `fortytwo_is_bigger_than_thirtytwo`
    // test runs the `_` case below instead of the second one
    // match a {
    //     a if a > b => return a,
    //     b if b > a => return b,
    //     _ => panic!(),
    // }
    // can't do below since it is another function call
    // max(a, b)
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}
