fn fizzbuzz(sz: usize) -> String {
    let div_3 = sz % 3 == 0;
    let div_5 = sz % 5 == 0;

    if div_3 && div_5 {
        String::from("fizzbuzz");
    } else if div_3 {
        String::from("fizz");
    } else if div_5 {
        String::from("buzz");
    } else {
        format!("{}", sz)
    }
}

fn main() {
    for i in 0..16 {
        println!("{}", fizzbuzz(i));
    }
}

