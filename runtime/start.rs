use std::env;

const I63_MAX : i64 = 4611686018427387903;
const I63_MIN : i64 = -4611686018427387904;

const TRUE_INT: i64 = 7;
const FALSE_INT: i64 = 3;
const NIL_INT: i64 = 1;

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
    let mut seen = Vec::<i64>::new();
    println!("{}", snek_to_str(val, &mut seen));
    val
}

#[export_name = "\x01snek_structural_equality"]
pub extern "C" fn x01snek_structural_equality(val1: i64, val2: i64) -> i64 {
    let mut seen = Vec::<(i64, i64)>::new();
    let struc_equality = check_structural_eq(val1, val2, &mut seen);
    if struc_equality == true {
        return TRUE_INT
    } else {
        return FALSE_INT
    }
}

fn snek_to_str(val: i64, seen : &mut Vec<i64>) -> String {
    if val == TRUE_INT { return format!("true"); }
    else if val == FALSE_INT { return format!("false"); }
    else if val % 2 == 0 {
        return format!("{}", (val >> 1));
    } else if val == NIL_INT {
        return format!("nil");
    } else if val & 1 == 1 { // indicates array
        if seen.contains(&val)  { return "(array <cyclic>)".to_string() }
        seen.push(val);
        let addr = (val - 1) as *const i64;
        let mut array_str = String::new();
        array_str.push_str("(");
        let size = unsafe { *addr };
        for i in 0..size {
            let value = unsafe { *addr.offset(i as isize + 1) };
            array_str.push_str(&snek_to_str(value, seen).to_string());
            if i != size - 1 {
                array_str.push_str(&", ".to_string());
            }
        }
        array_str.push_str(")");
        seen.pop();
        return format!("{}", array_str);
    }
    else {
        return format!("Unknown value: {}", val);
    }
}

fn check_structural_eq(val1: i64, val2: i64, seen: &mut Vec<(i64, i64)>) -> bool {
    if ((val1 & 1) == 0 && (val2 & 1) == 0) || ((val1 & 3) == 3 && (val2 & 3) == 3) {
        val1 == val2
    } else if (val1 & 3) == 1 && (val2 & 3) == 1 { // arrays 
        if val1 == val2 {
            return true;
        } else if seen.contains(&(val1, val2)) {
            return true;
        }
        else {
            seen.push((val1, val2));
            let addr_v1 = (val1 - 1) as *const i64;
            let addr_v2 = (val2 - 1) as *const i64;
            let size_v1 = unsafe { *addr_v1.offset(0) };
            let size_v2 = unsafe { *addr_v2.offset(0) };
            if size_v1 != size_v2 {
                seen.pop();
                return false;
            }
            for i in 0..size_v1 {
                let value_v1 = unsafe { *addr_v1.offset(i as isize + 1) };
                let value_v2 = unsafe { *addr_v2.offset(i as isize + 1) };
                if !check_structural_eq(value_v1, value_v2, seen) {
                    seen.pop();
                    return false;
                }
            }
        }
        seen.pop();
        return true;
    } else { // type mismatch
        return false;
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
    let mut seen = Vec::<i64>::new();
    println!("{}", snek_to_str(i, &mut seen));
}
fn main() {
    let args: Vec<String> = env::args().collect();

    let input = if args.len() == 2 { &args[1] } else { "false" };
    let input = parse_input(&input);

    let mut memory = Vec::<i64>::with_capacity(1000000);
    let buffer : *mut i64 = memory.as_mut_ptr();
    // println!("buffer addr - {:p}", buffer);

    let i: i64 = unsafe { our_code_starts_here(input, buffer) };
    print_value(i);
}
