use reqwest;

fn main() {
  println!("Hello, world!");

  let res = reqwest::blocking::get("https://www.rust-lang.org");
  if res.is_err() {
    println!("found error");
    return;
  }



  // println!("body = {:?}", body);

  // let resp = reqwest::blocking::get("https://httpbin.org/ip")?.text()?;
  // println!("{:#?}", resp);
  // Ok(())
}
