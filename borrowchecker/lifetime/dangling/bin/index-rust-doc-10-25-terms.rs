fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        //result = longest(string1.as_str(), string2.as_str());
        result = longest(string1, string2);
        println!("The longest string is {}", result);
    }
    
}
/* no error
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
In this example, we’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, because the lifetime of y does not have any relationship with the lifetime of x or the return value.

When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters. If the reference returned does not refer to one of the parameters, it must refer to a value created within this function, which would be a dangling reference because the value will go out of scope at the end of the function
*/

/*error
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    let result = String::from("really long string");
    *Important: result.as_str()//no relation between params lifetime and return, so we won't need any lifetime generic.
}
fn longest<'a>(x: & str, y: & str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
*/

/* no error
fn longest(x: String, y: String) -> String {
    let result = String::from("really long string");
    result
}

*/