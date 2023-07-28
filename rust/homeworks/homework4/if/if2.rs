// if2.rs

// Step 1: Make me compile!
// Step 2: Get the bar_for_fuzz and default_to_baz tests passing!
// Execute the command `zustlings hint if2` if you want a hint :)

// https://stackoverflow.com/questions/58862093/returns-a-value-referencing-data-owned-by-the-current-function
pub fn fizz_if_foo(fizzish: &str) -> String {
    // if fizzish == "fizz" {
    //     "foo".to_string()
    // }
    // else if fizzish == "fuzz" {
    //     "bar".to_string()
    // }
    // else {
    //     // 1.to_string()
    //     "baz".to_string()
    // }
    match fizzish {
        "fizz" => "foo".to_string(),
        "fuzz" => "bar".to_string(),
        _ => "baz".to_string(),
    }
}

// No test changes needed!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo_for_fizz() {
        assert_eq!(fizz_if_foo("fizz"), "foo")
    }

    #[test]
    fn bar_for_fuzz() {
        assert_eq!(fizz_if_foo("fuzz"), "bar")
    }

    #[test]
    fn default_to_baz() {
        assert_eq!(fizz_if_foo("literally anything"), "baz")
    }
}
