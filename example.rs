mod cnot;
mod spar;

fn main() {
    cnot::rebuild_edition(
        &mut std::env::args(),
        cnot::RustEdition::R2018,
        &["example.rs", "cnot.rs", "spar.rs"],
    );
    cnot::generate_project("example.rs", cnot::RustEdition::R2018).unwrap();

    let bool_test   = spar::flag_bool("bool", false);
    let long_test   = spar::flag_long("long", 0);
    let ulong_test  = spar::flag_ulong("ulong", 0);
    let float_test  = spar::flag_float("float", 0.0);
    let double_test = spar::flag_double("double", 0.0);
    let string_test = spar::flag_string("string", "");
    spar::parse_args(&mut std::env::args());
    println!("{} = {}", bool_test.name(), bool_test.value());
    println!("{} = {}", long_test.name(), long_test.value());
    println!("{} = {}", ulong_test.name(), ulong_test.value());
    println!("{} = {}", float_test.name(), float_test.value());
    println!("{} = {}", double_test.name(), double_test.value());
    println!("{} = {}", string_test.name(), string_test.value());
}
