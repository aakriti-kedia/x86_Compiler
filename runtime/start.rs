use std::env;

const I63_MAX : i64 = 4611686018427387903;
const I63_MIN : i64 = -4611686018427387904;

const TRUE_INT: i64 = 7;
const FALSE_INT: i64 = 3;
const NIL_INT: i64 = 9;

#[link(name = "our_code")]
extern "C" {
    // The \x01 here is an undocumented feature of LLVM that ensures
    // it does not add an underscore in front of the name.
    // Courtesy of Max New (https://maxsnew.com/teaching/eecs-483-fa22/hw_adder_assignment.html)
    #[link_name = "\x01our_code_starts_here"]
    fn our_code_starts_here(input: i64, memory: *mut i64) -> i64;
}

#[export_name = "\x01snek_error"]
pub extern "C" fn snek_error(errcode: i64) {
    match errcode {
        7 => eprintln!("invalid argument: type mismatch"),
        8 => eprintln!("overflow on operation"),
        9 => eprintln!("index_out_of_bounds_error for array"),
        err_code => eprintln!("invalid argument: random error {err_code}"),
    }
    std::process::exit(1);
}

#[export_name = "\x01snek_print"]
pub extern "C" fn snek_print(val: i64) -> i64 {
    println!("{}", snek_to_str(val));
    val
}

fn snek_to_str(val: i64) -> String {
    if val == TRUE_INT { return format!("true"); }
    else if val == FALSE_INT { return format!("false"); }
    else if val % 2 == 0 {
        return format!("{}", (val >> 1));
    } else if val == NIL_INT {
        return format!("nil");
    } else if val & 1 == 1 { // indicates array / list / tuple / multiple values
        // println!("array addr - {}", val);
        let addr = (val - 1) as *const i64;
        let mut array_str = String::new();
        array_str.push_str("(");
        let size = unsafe { *addr };
        // println!("size - {}", size);
        for i in 0..size {
            let value = unsafe { *addr.offset(i as isize + 1) };
            array_str.push_str(&snek_to_str(value).to_string());
            if i != size - 1 {
                array_str.push_str(&", ".to_string());
            }
        }
        array_str.push_str(")");
        return format!("{}", array_str);
    }
    else {
        return format!("Unknown value: {}", val);
    }
}

fn parse_input(input: &str) -> i64 {
    // TODO: parse the input string into internal value representation
    if input == "true" {
        TRUE_INT
    } else if input == "false" {
        FALSE_INT
    } else {
        match input.parse::<i64>() {
            Ok(num) => {
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
    println!("{}", snek_to_str(i));
}
fn main() {
    let args: Vec<String> = env::args().collect();

    let input = if args.len() == 2 { &args[1] } else { "false" };
    let input = parse_input(&input);

    let mut memory = Vec::<i64>::with_capacity(1000000);
    let buffer : *mut i64 = memory.as_mut_ptr();

    let i: i64 = unsafe { our_code_starts_here(input, buffer) };
    print_value(i);
}
