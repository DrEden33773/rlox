#[test]
fn load_meaningless_lox() {
  use project_root::get_project_root;
  use std::fs::read_to_string;
  let project_root = get_project_root().expect("There is no project root");
  let meaningless_path = project_root.to_str().unwrap().to_owned() + "/private/src/meaningless.lox";
  let meaningless_string = read_to_string(meaningless_path).unwrap();
  println!("{}", meaningless_string);
}
