use jep106::JEP106Code;

fn main() {

    println!("Manufacturer is [{}]", JEP106Code::new(0x04, 0x3b));
    println!("Manufacturer is [{}]", JEP106Code::new(0x08, 0x19));
    println!("Manufacturer is [{}]", JEP106Code::new(0x0, 0x1));
    println!("Manufacturer is [{}]", JEP106Code::new(0x0, 0x49));
    println!("Manufacturer is [{}]", JEP106Code::new(0x0, 0x70));
}
