fn main() {
    let input = ["once  upon a time", "there  was an example"];
    let mut input_iter = input.iter();
    let mut words = vec![];
    for _ in 0..input.len() {
        words.push(
            input_iter
                .next()
                .as_mut()
                .unwrap()
                .split(' ')
                .filter(|x| *x != "")
                .collect::<Vec<_>>(),
        );
    }
    println!("{:?}", words);
}