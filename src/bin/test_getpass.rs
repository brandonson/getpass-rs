extern crate getpass;

fn main() {
  let pass = getpass::get_pass("Enter a password:");
  println!("{}", pass.unwrap());
}
