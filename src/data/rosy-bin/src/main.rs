use rosy::prelude::*;

pub fn main() {
    rosy::vm::init().expect("Could not initialize Ruby");
    let class = Class::get("Object").unwrap();
    let obj = class.new_instance().unwrap();
    unsafe { obj.call_with("puts", &[String::from("Hello, Ruby world!")]) };
}
