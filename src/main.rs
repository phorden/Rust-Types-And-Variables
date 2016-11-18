const MEANING_OF_LIFE:u8 = 42; // no fixed address

static mut Z:i32 = 123; // like const, but with memory address, generally consts are still prefered.

fn scope_and_shadowing() {
    let a = 123;

    // anonymous scope
    {
        let b = 456;
        println!("inside, b = {}", b);

        let a = 777;

        println!("inside, a = {}", a);
    }
    // println!("inside, b = {}", b); does not exist outside of the above scope

    println!("outside, a = {}", a);
}

fn operators() {
    // arithmetic

    let mut a = 2 + 3 * 4;  // + - * /
    println!("{}", a);
    a = a + 1; // no -- or ++ operators
    a -= 2; // a = a -2;
     // -= += *= /+ %/

    println!("remainder of {} / {} = {}", a, 3, (a%3));

    let a_cubed = i32::pow(a, 3);  // power numbers
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5; // floating point number
    let b_cubed = f64::powi(b, 3); // intiger power
    let b_to_pi = f64::powf(b, std::f64::consts::PI); // floating power
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    // bitwise
    let c = 1 | 2; // | OR & AND ^ XOR ! NOR
                    // 01 OR 10 = 11 == 3_10
    println!("1|2 = {}", c);
    let two_to_ten = 1 << 10;
    println!("2^10 = {}", two_to_ten);

    //logical operators
    let pi_less_four = std::f64::consts::PI < 4.0; // true
    // > <= >=
    let x = 5;
    let x_is_five = x == 5; // true
}

fn main() {
    println!("{}", MEANING_OF_LIFE);
    // operators
    // operators();
    // scope_and_shadowing();

    // can allow mut vars to still run
    unsafe {
        println!("{}", Z);
    }
}
