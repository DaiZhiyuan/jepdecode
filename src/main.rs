use jep106::JEP106Code;

fn printManufacturer(this: JEP106Code) {
    println!("Manufacturer is [{}]", this);
}

fn main() {
    printManufacturer(JEP106Code::new(0x4, 0x3b));
    printManufacturer(JEP106Code::new(0x8, 0x19));
}
