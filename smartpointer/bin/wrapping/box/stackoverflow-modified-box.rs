fn modifier2(mut ptr: Box<String>) -> Box<String> {
    println!("In modifier2...");
    println!("Ptr points to {:p}, and value is {}", ptr, *ptr);

    *ptr = ptr.to_uppercase();

    println!("Exit modifier2...");
    ptr
}

fn modifier3(ptr: &mut Box<String>) {
    println!("In modifier3...");
    println!("Ptr points to {:p}, and value is {}", ptr, *ptr);
    println!("Ptr points to {:p}, and value is {}", *ptr, **ptr);

    **ptr = "another".to_uppercase();

    **ptr = ptr.to_uppercase(); // No error now

    println!("Exit modifier3...");
}

fn main() {
    let mut answer = Box::new("Hello World".to_string());
    answer = modifier2(answer);
    println!("called modifier2(): {} length: {}", answer, answer.len());

    let mut answer = Box::new("Hello World".to_string());
    modifier3(&mut answer);
    println!("called modifier3(): {} length: {}", answer, answer.len());
}