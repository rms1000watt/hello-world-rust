use reqwest;

fn main() {
  println!("Hello, world!");

  // ref: https://www.reddit.com/r/learnrust/comments/v3sjwg/comment/ib0avhz
  let result = reqwest::blocking::get("https://www.rust-lang.org");
  let response = match result {
    Ok(response) => response,
    Err(e) => {
      println!("failed making request {}", e);
      return;
    }
  };

  let text = match response.text() {
    Ok(text) => text,
    Err(e) => {
      println!("failed getting text {}", e);
      return;
    }
  };

  println!("{}",text);
}
