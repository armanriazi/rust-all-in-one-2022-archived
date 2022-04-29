use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Reqwest(reqwest::Error);
        ParseIntError(std::num::ParseIntError);
    }
    errors { RandomResponseError(t: String) }
}

fn parse_response(response: reqwest::blocking::Response) -> Result<u32> {
  let mut body = response.text()?;
  body.pop();
  body
    .parse::<u32>()
    .chain_err(|| ErrorKind::RandomResponseError(body))
}

fn run() -> Result<()> {
  let url =
    format!("https://www.random.org/integers/?num=1&min=0&max=10&col=1&base=10&format=plain");
  let response = reqwest::blocking::get(&url)?;
  let random_value: u32 = parse_response(response)?;
  println!("a random number between 0 and 10: {}", random_value);
  Ok(())
}

fn main() {
  if let Err(error) = run() {
    match *error.kind() {
      ErrorKind::Io(_) => println!("Standard IO error: {:?}", error),
      ErrorKind::Reqwest(_) => println!("Reqwest error: {:?}", error),
      ErrorKind::ParseIntError(_) => println!("Standard parse int error: {:?}", error),
      ErrorKind::RandomResponseError(_) => println!("User defined error: {:?}", error),
      _ => println!("Other error: {:?}", error),
    }
  }
}