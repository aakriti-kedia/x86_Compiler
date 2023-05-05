use std::env;

const I63_MAX : i64 = 4611686018427387903;
const I63_MIN : i64 = -4611686018427387904;

#[link(name = "our_code")]
extern "C" {
    // The \x01 here is an undocumented feature of LLVM that ensures
    // it does not add an underscore in front of the name.
    // Courtesy of Max New (https://maxsnew.com/teaching/eecs-483-fa22/hw_adder_assignment.html)
    #[link_name = "\x01our_code_starts_here"]
    fn our_code_starts_here(input: i64) -> i64;
}

#[export_name = "\x01snek_error"]
pub extern "C" fn snek_error(errcode: i64) {
    match errcode {
        7 => eprintln!("invalid argument: type mismatch"),
        8 => eprintln!("overflow on operation"),
        _ => eprintln!("invalid argument: random error"),
    }
    std::process::exit(1);
}

// // how to detect num of bits required to represent number 
// fn num_bits_required(num: i64) -> u32 {
//     let u = num.abs() as u64;
//     64 - u.leading_zeros()
// }

fn parse_input(input: &str) -> i64 {
    // TODO: parse the input string into internal value representation
    if input == "true" {
        3
    } else if input == "false" {
        1
    } else {
        match input.parse::<i64>() {
            Ok(num) => {

                // let n_bits = num_bits_required(num);
                // println!("n_bits - {}", n_bits);
                // if num_bits_required(num) > 62 {
                if num > I63_MAX || num < I63_MIN {
                    panic!("Invalid: num can't fit in 63 bits")
                } else {
                    num << 1
                }
            },
            Err(_) => panic!("Invalid: num can't fit in 63 bits")
        }
    }
}

fn print_value(i: i64) {
    // println!("print_value {}", i);
    if i % 2 == 0 {
        println!("{}", (i >> 1));
    } else if i == 3 {
        println!("true");
    } else if i == 1 {
        println!("false");
    } else {
        println!("unknown");
    } 
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() == 2 { &args[1] } else { "false" };
    let input = parse_input(&input);

    // println!("input - {}", input);

    let i: i64 = unsafe { our_code_starts_here(input) };
    print_value(i);
}
