// modules2.rs
// You can bring module paths into scopes and provide new names for them with the
// 'use' and 'as' keywords. Fix these 'use' statements to make the code compile.
// Make me compile! Execute `rustlings hint modules2` for hints :)

mod delicious_snacks {

    // Fix: these are fine if used inside this module
    use self::fruits::PEAR as fruit;
    use self::veggies::CUCUMBER as veggie;

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }

    pub fn print_snacks() {
        println!("Fruit: {}", fruit);
        println!("Veggie: {}", veggie);
    }
}
fn main() {
    delicious_snacks::print_snacks();
}
