use std::env;
use std::fs::File;
use std::io::prelude::*;

use sexp::Atom::*;
use sexp::*;

use im::HashMap;
use std::collections::HashSet;

const TRUE_INT :i64 = 3;
const FALSE_INT :i64 = 1;
const INT_ONE :i64 = 2;

const I63_MAX : i64 = 4611686018427387903;
const I63_MIN : i64 = -4611686018427387904;

#[derive(Debug, Clone)]
struct Program {
  defs: Vec<Definition>,
  main: Expr,
}

#[derive(Debug, Clone)]
struct Definition {
  fun_name: String, 
  args: Vec<String>, 
  fun_body: Expr,
}

#[derive(Debug, Clone)]
enum Val {
    Reg(Reg),
    Imm(i64),
    RegOffset(Reg, i32),
}

#[derive(Debug, Clone)]
enum Reg {
    RAX,
    RBX,
    RSP,
    RDI,
}

#[derive(Debug, Clone)]
enum Instr {
    IMov(Val, Val),
    IAdd(Val, Val),
    ISub(Val, Val),
    IMul(Val, Val),
    ICmp(Val, Val),
    IXor(Val, Val),
    ITest(Val, Val),
    ICmove(Val, Val),
    IJg(String),
    IJge(String),
    IJl(String),
    IJle(String),
    ILabel(String, Vec<Instr>),
    IJe(String),
    IJmp(String),
    IJne(String),
    IPush(Val),
    IPop(Val),
    // IJz(String),
    // IJnz(String),
    // ILoop(String), 
    // IBreak(Val),
    // IRet(),
    ISar(Val, u64), 
    IJc(String), 
    ICall(String),
    // IJnc(String),
    IJo(String),
    ICmovb(Val, Val),
    // IComment(String),

}

#[derive(Debug, Clone)]
enum Op1 {
    Add1,
    Sub1,
    IsNum, 
    IsBool,
    Print,
}

#[derive(Debug, Clone)]
enum Op2 {
    Plus,
    Minus,
    Times,
    Equal, 
    Greater, 
    GreaterEqual, 
    Less, 
    LessEqual,
}

#[derive(Clone, Debug)]
enum Expr {
    Number(i64),
    True, 
    False,
    Input,
    Id(String),
    Let(Vec<(String, Expr)>, Box<Expr>),
    UnOp(Op1, Box<Expr>),
    BinOp(Op2, Box<Expr>, Box<Expr>),
    Set(String, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>), 
    Block(Vec<Expr>), 
    Loop(Box<Expr>), 
    Break(Box<Expr>),

    Function(String, Vec<Expr>),
}

// helpers 

fn new_label(l: &mut i32, s: &str) -> String {
  let current = *l;
  *l += 1;
  format!("{s}_{current}")
}

fn check_overflow(_v: Val) -> Vec<Instr> {
  let mut instr_vect = Vec::new();
  instr_vect.push(Instr::IJo("throw_overflow_error".to_string()));
  instr_vect
}

fn check_if_num(v: Val) -> Vec<Instr> {
  let mut instr_vect = Vec::new();
  instr_vect.push(Instr::IMov(Val::Reg(Reg::RBX), v));
  instr_vect.push(Instr::ISar(Val::Reg(Reg::RBX), 1)); // should have 0
  instr_vect
}

fn check_input_num(v: Val) -> Vec<Instr> {
  let mut instr_vect = check_if_num(v);
  instr_vect.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(3)));
  instr_vect.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(1)));
  instr_vect.push(Instr::ICmovb(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
  instr_vect
}

fn is_num(v: Val) -> Vec<Instr> {
  let mut instr_vect = check_if_num(v);
  instr_vect.push(Instr::IJc("throw_error".to_string()));
  instr_vect
}

fn check_input_bool(v: Val) -> Vec<Instr> {
  let mut instr_vect = check_if_bool(v);
  instr_vect.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(1)));
  instr_vect.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(3)));
  instr_vect.push(Instr::ICmovb(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
  instr_vect
}

fn check_if_bool(v: Val) -> Vec<Instr> {
  let mut instr_vect = Vec::new();
  instr_vect.push(Instr::IMov(Val::Reg(Reg::RBX), v));
  instr_vect.push(Instr::ISar(Val::Reg(Reg::RBX), 1)); // should have 1
  instr_vect
}

fn check_type_match(v1: Val, v2: Val) -> Vec<Instr> {
  let mut instr_vect = Vec::new();
  instr_vect.push(Instr::IMov(Val::Reg(Reg::RBX), v1));
  instr_vect.push(Instr::IXor(Val::Reg(Reg::RBX), v2));
  instr_vect.push(Instr::ITest(Val::Reg(Reg::RBX), Val::Imm(FALSE_INT)));
  instr_vect.push(Instr::IJne("throw_error".to_string()));
  instr_vect
}

// instr to string helpers
fn instr_to_str(i: &Instr) -> String {
  match i {
      Instr::IMov(v1, v2) => format!("mov {}, {}", val_to_str(v1), val_to_str(v2)),
      Instr::IAdd(v1, v2) => format!("add {}, {}", val_to_str(v1), val_to_str(v2)),
      Instr::ISub(v1, v2) => format!("sub {}, {}", val_to_str(v1), val_to_str(v2)),
      Instr::IMul(v1, v2) => format!("imul {}, {}", val_to_str(v1), val_to_str(v2)),
      Instr::ICmp(v1, v2) => format!("cmp {}, {}", val_to_str(v1), val_to_str(v2)),
      Instr::ICmove(v1, v2) => format!("cmove {}, {}", val_to_str(v1), val_to_str(v2)),
      Instr::IJg(label) => format!("jg {}", label),
      Instr::IJge(label) => format!("jge {}", label),
      Instr::IJl(label) => format!("jl {}", label),
      Instr::IJle(label) => format!("jle {}", label),
      Instr::ICall(fun_name) => format!("call {}", fun_name),
      Instr::IPush(v1) => format!("push {}", val_to_str(v1)),
      Instr::IPop(v1) => format!("pop {}", val_to_str(v1)),
      // Instr::IJz(label) => format!("jz {}", label),
      // Instr::IJnz(label) => format!("jnz {}", label),
      Instr::ILabel(label, instrs) => format!("{}:{}", label, decode_instrs_vec_to_string(instrs)),
      Instr::IJe(label) => format!("je {}", label),
      Instr::IJc(label) => format!("jc {}", label),
      Instr::IJo(label) => format!("jo {}", label),
      // Instr::IJnc(label) => format!("jnc {}", label),
      Instr::ISar(v1, bits) => format!("sar {}, {}", val_to_str(v1), bits),
      Instr::IJmp(label) => format!("jmp {}", label),
      Instr::IJne(label) => format!("jne {}", label),
      Instr::IXor(v1, v2) => format!("xor {}, {}", val_to_str(v1), val_to_str(v2)),
      Instr::ITest(v1, v2) => {
        match v1 {
          Val::RegOffset(_, _) => format!("test word{}, {}", val_to_str(v1), val_to_str(v2)),
          _ => format!("test {}, {}", val_to_str(v1), val_to_str(v2))
        }
      },
      // Instr::ILoop(label) => format!("loop {}", label),
      // Instr::IBreak(v1) => format!("break {}", val_to_str(v1)),
      Instr::ICmovb(v1, v2) => format!("cmovb {}, {}", val_to_str(v1), val_to_str(v2)),
      // Instr::IComment(comment) => format!("; {}", comment),
      // Instr::IRet() => format!("ret"),
  }
}

fn reg_str(reg_name: &Reg) -> String {
match reg_name {
    Reg::RAX => "rax", 
    Reg::RSP => "rsp", 
    Reg::RBX => "rbx", 
    Reg::RDI => "rdi", 
}.to_string()
}

fn val_to_str(v: &Val) -> String {
  match v {
      Val::Reg(reg_name) => {
        reg_str(reg_name)
      },
      Val::Imm(num) => num.to_string(),
      Val::RegOffset(reg_name, stack_offset) => {
          let reg_name = reg_str(reg_name);
          format!("[{} - {}]", reg_name, stack_offset)
      },
  }
}

fn decode_instrs_vec_to_string(vect_instrs: &Vec<Instr>) -> String {
let mut ans = "".to_owned();
for inst in vect_instrs.iter() {
    ans += &("\n".to_owned() + &instr_to_str(&inst));
    // ans.push_str("\n");
    // ans.push_str(&instr_to_str(&inst));
}
ans.to_string()
}

// -------

// compiling
 
fn compile_to_instrs(e: & Expr, si: i32, env: & HashMap<String, i32>, break_label: &String, l: &mut i32) -> Vec<Instr> {
    let mut instr_vector = Vec::new();
    match e {
        Expr::Number(n) => {
            instr_vector.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(*n)));
            instr_vector.extend(check_overflow(Val::Reg(Reg::RAX)));
        }, 
        Expr::True => {
          instr_vector.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(TRUE_INT)))
        },
        Expr::False => {
          instr_vector.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(FALSE_INT)))
        },
        Expr::Input => {
          instr_vector.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Reg(Reg::RDI)));
          instr_vector.extend(check_overflow(Val::Reg(Reg::RAX)))
        },
        Expr::Id(s) => {
            println!("env {:?}, s = {}", *env, s);
            if env.contains_key(s) {
                instr_vector.push(Instr::IMov(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, *env.get(s).unwrap())))
            } else {
                panic!("Unbound variable identifier {}", s);
            }
        },
        Expr::Let(binding, body) => {
            let mut nenv = env.clone();
            let mut si_local = si;
            let mut current_binding = HashSet::new();
            if binding.len() < 1 {
                panic!("Invalid");
            }
            for (var_id, value) in binding.iter() {
                println!("compile_to_instrs - {}", var_id);
                if current_binding.contains(var_id) {
                    panic!("Duplicate binding");
                }
                current_binding.insert(var_id);
                let binding_expr_instrs = compile_to_instrs(value, si_local, &nenv, &break_label, l);
                nenv = nenv.update(var_id.to_string(), si_local * 8);
                println!("nenv {:?}", nenv);
                instr_vector.extend(binding_expr_instrs);
                let curr_instr = Instr::IMov(Val::RegOffset(Reg::RSP, si_local * 8), Val::Reg(Reg::RAX));
                instr_vector.push(curr_instr);
                si_local += 1;
            }

            let body_instrs = compile_to_instrs(body, si_local, &nenv, &break_label, l);
            instr_vector.extend(body_instrs);
        }, 
        Expr::UnOp(op1, subexp) => {
            let subexp_instrs = compile_to_instrs(subexp, si, &env, &break_label, l);
            instr_vector.extend(subexp_instrs);
            match op1 {
                Op1::Add1 => {
                  let v1 = Val::Reg(Reg::RAX);
                  let v2 = Val::Imm(INT_ONE);
                  instr_vector.extend(is_num(v1.clone()));
                  instr_vector.extend(is_num(v2.clone()));
                  instr_vector.push(Instr::IAdd(v1.clone(), v2.clone()));
                  instr_vector.extend(check_overflow(v1.clone()));
                }
                Op1::Sub1 => {
                  let v1 = Val::Reg(Reg::RAX);
                  let v2 = Val::Imm(INT_ONE);
                  instr_vector.extend(is_num(v1.clone()));
                  instr_vector.extend(is_num(v2.clone()));
                  instr_vector.push(Instr::ISub(v1.clone(), v2.clone()));
                  instr_vector.extend(check_overflow(v1.clone()));
                }
                Op1::IsNum => {
                  instr_vector.extend(check_input_num(Val::Reg(Reg::RAX)));
                },
                Op1::IsBool => {
                  instr_vector.extend(check_input_bool(Val::Reg(Reg::RAX)));
                },
                Op1::Print => {
                  // sub offset for RSP
                  let offset = si * 8;
                  instr_vector.push(Instr::ISub(Val::Reg(Reg::RSP), Val::Imm(offset.into())));
        
                  // store rdi on stack 
                  instr_vector.push(Instr::IPush(Val::Reg(Reg::RDI)));
        
                  // put rax in rdi
                  instr_vector.push(Instr::IMov(Val::Reg(Reg::RDI), Val::Reg(Reg::RAX)));
        
                  // call snek_print fun
                  instr_vector.push(Instr::ICall("snek_print".to_owned()));
        
                  // restore RDI 
                  instr_vector.push(Instr::IPop(Val::Reg(Reg::RDI)));
        
                  // add back offset
                  instr_vector.push(Instr::IAdd(Val::Reg(Reg::RSP), Val::Imm(offset.into())));
                }
            };
        }, 
        Expr::BinOp(op2, subexp1, subexp2) => {
            let e1_instrs = compile_to_instrs(subexp1, si, &env, &break_label, l);
            let e2_instrs = compile_to_instrs(subexp2, si+1, &env, &break_label, l);
            let stack_offset = si * 8;
            instr_vector.extend(e1_instrs);
            instr_vector.push(Instr::IMov(Val::RegOffset(Reg::RSP, stack_offset), Val::Reg(Reg::RAX)));
            instr_vector.extend(e2_instrs);

            match op2 {
                Op2::Plus => {
                  let v1 = Val::Reg(Reg::RAX);
                  let v2 = Val::RegOffset(Reg::RSP, stack_offset);
                  instr_vector.extend(is_num(v1.clone()));
                  instr_vector.extend(is_num(v2.clone()));
                  // instr_vector.push(Instr::IRet());
                  instr_vector.push(Instr::IAdd(v1.clone(), v2.clone()));
                  instr_vector.extend(check_overflow(v1.clone()));
                }
                Op2::Minus => {
                  let v1 = Val::RegOffset(Reg::RSP, stack_offset);
                  let v2 = Val::Reg(Reg::RAX);
                  instr_vector.extend(is_num(v1.clone()));
                  instr_vector.extend(is_num(v2.clone()));
                  instr_vector.push(Instr::ISub(v1.clone(), v2.clone()));
                  instr_vector.push(Instr::IMov(v2.clone(), v1.clone()));
                  instr_vector.extend(check_overflow(v2.clone()));
                }
                Op2::Times => {
                  let v1 = Val::Reg(Reg::RAX);
                  let v2 = Val::RegOffset(Reg::RSP, stack_offset);
                  instr_vector.extend(is_num(v1.clone()));
                  instr_vector.extend(is_num(v2.clone()));
                  instr_vector.push(Instr::ISar(v1.clone(), 1));
                  instr_vector.push(Instr::IMul(v1.clone(), v2.clone()));
                  // instr_vector.push(Instr::IRet()); 
                  instr_vector.extend(check_overflow(v1.clone()));
                }
                Op2::Equal => {
                    instr_vector.extend(check_type_match(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, stack_offset)));
                    instr_vector.push(Instr::ICmp(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, stack_offset)));
                    instr_vector.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(TRUE_INT)));
                    instr_vector.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(FALSE_INT)));
                    instr_vector.push(Instr::ICmove(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
                }
                Op2::Greater => {
                    let greater_label = new_label(l, "greater");
                    let greater_end_label = new_label(l, "greater_end");

                    let v1 = Val::Reg(Reg::RAX);
                    let v2 = Val::RegOffset(Reg::RSP, stack_offset);
                    instr_vector.extend(is_num(v1.clone()));
                    instr_vector.extend(is_num(v2.clone()));

                    instr_vector.push(Instr::ICmp(v1.clone(), v2.clone()));
                    instr_vector.push(Instr::IJl(greater_label.clone())); // comparing 2nd with 1st param
                    instr_vector.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(FALSE_INT)));
                    instr_vector.push(Instr::IJmp(greater_end_label.clone()));

                    let mut block_instrs = Vec::new();
                    block_instrs.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(TRUE_INT)));

                    instr_vector.push(Instr::ILabel(greater_label, block_instrs));
                    instr_vector.push(Instr::ILabel(greater_end_label, Vec::new()));
                }
                Op2::Less => { 

                  let less_label = new_label(l, "less");
                  let less_end_label = new_label(l, "less_end");

                  let v1 = Val::Reg(Reg::RAX);
                  let v2 = Val::RegOffset(Reg::RSP, stack_offset);
                  instr_vector.extend(is_num(v1.clone()));
                  instr_vector.extend(is_num(v2.clone()));

                  instr_vector.push(Instr::ICmp(v1.clone(), v2.clone()));
                  instr_vector.push(Instr::IJg(less_label.clone())); // comparing 2nd with 1st param
                  instr_vector.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(FALSE_INT)));
                  instr_vector.push(Instr::IJmp(less_end_label.clone()));

                  let mut block_instrs = Vec::new();
                  block_instrs.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(TRUE_INT)));

                  instr_vector.push(Instr::ILabel(less_label, block_instrs));
                  instr_vector.push(Instr::ILabel(less_end_label, Vec::new()));

                }
                Op2::GreaterEqual => {
                  let greater_eq_label = new_label(l, "greater_eq");
                  let greater_eq_end_label = new_label(l, "greater_eq_end");

                  let v1 = Val::Reg(Reg::RAX);
                  let v2 = Val::RegOffset(Reg::RSP, stack_offset);
                  instr_vector.extend(is_num(v1.clone()));
                  instr_vector.extend(is_num(v2.clone()));

                  instr_vector.push(Instr::ICmp(v1.clone(), v2.clone()));
                  instr_vector.push(Instr::IJle(greater_eq_label.clone())); // comparing 2nd with 1st param
                  instr_vector.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(FALSE_INT)));
                  instr_vector.push(Instr::IJmp(greater_eq_end_label.clone()));

                  let mut block_instrs = Vec::new();
                  block_instrs.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(TRUE_INT)));

                  instr_vector.push(Instr::ILabel(greater_eq_label, block_instrs));
                  instr_vector.push(Instr::ILabel(greater_eq_end_label, Vec::new()));
                }
                Op2::LessEqual => {
                  let less_eq_label = new_label(l, "less_eq");
                  let less_eq_end_label = new_label(l, "less_eq_end");

                  let v1 = Val::Reg(Reg::RAX);
                  let v2 = Val::RegOffset(Reg::RSP, stack_offset);
                  instr_vector.extend(is_num(v1.clone()));
                  instr_vector.extend(is_num(v2.clone()));

                  instr_vector.push(Instr::ICmp(v1.clone(), v2.clone()));
                  instr_vector.push(Instr::IJge(less_eq_label.clone())); // comparing 2nd with 1st param
                  instr_vector.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(FALSE_INT)));
                  instr_vector.push(Instr::IJmp(less_eq_end_label.clone()));

                  let mut block_instrs = Vec::new();
                  block_instrs.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(TRUE_INT)));

                  instr_vector.push(Instr::ILabel(less_eq_label, block_instrs));
                  instr_vector.push(Instr::ILabel(less_eq_end_label, Vec::new()));
                }

            }
        }
        Expr::If(cond_exp, then_exp, else_exp) => {
          let end_label = new_label(l, "ifend");
          let else_label = new_label(l, "ifelse");
          let cond_instrs = compile_to_instrs(cond_exp, si, &env, &break_label, l);
          let then_instrs = compile_to_instrs(then_exp, si, &env, &break_label, l);
          let else_instrs = compile_to_instrs(else_exp, si, &env, &break_label, l);
          instr_vector.extend(cond_instrs);
          instr_vector.push(Instr::ICmp(Val::Reg(Reg::RAX), Val::Imm(FALSE_INT)));
          instr_vector.push(Instr::IJe(else_label.clone()));
          instr_vector.extend(then_instrs);
          instr_vector.push(Instr::IJmp(end_label.clone()));
          instr_vector.push(Instr::ILabel(else_label, else_instrs));
          instr_vector.push(Instr::ILabel(end_label, Vec::new()));
        }
        Expr::Set(var_name, exp) => {
          let subexp_instrs = compile_to_instrs(exp, si, &env, &break_label, l);
          instr_vector.extend(subexp_instrs);
          if env.contains_key(var_name) {
            let var_st_pos = *env.get(var_name).unwrap();
            instr_vector.push(Instr::IMov(Val::RegOffset(Reg::RSP, var_st_pos), Val::Reg(Reg::RAX)));
          } else {
            panic!("Unbound variable identifier {}", var_name);
          }
        }
        Expr::Block(block_instrs_expr) => {
          if block_instrs_expr.len() < 1 {
            panic!("Invalid");
          }
          for instr_expr in block_instrs_expr.iter() {
            instr_vector.extend(compile_to_instrs(instr_expr, si, &env, &break_label, l));
          }
        }
        Expr::Loop(loop_instrs) => {

          let start_loop = new_label(l, "loop");
          let end_loop = new_label(l, "loopend");
          let loop_instrs = compile_to_instrs(loop_instrs, si, &env, &end_loop, l);
          instr_vector.push(Instr::ILabel(start_loop.clone(), loop_instrs));
          instr_vector.push(Instr::IJmp(start_loop));
          instr_vector.push(Instr::ILabel(end_loop.clone(), Vec::new()));
        }
        Expr::Break(break_instrs) => {
          if break_label == "" {
            panic!("break outside loop");
          }
          instr_vector.extend(compile_to_instrs(break_instrs, si, &env, &break_label, l));
          instr_vector.push(Instr::IJmp(break_label.clone()));
        },
        Expr::Function(_, _) => todo!(),
    }
    instr_vector.to_vec()
}

fn compile(e: &Expr) -> String {
    let si = 2;
    let env = im::HashMap::<String, i32>::new();
    let mut l = 0;
    let break_label = "";
    let vect_instrs = compile_to_instrs(&e, si, &env, &break_label.to_owned(), &mut l);
    decode_instrs_vec_to_string(&vect_instrs)
}


// parsing 

fn parse_bind(s: &Sexp,defs: &Vec<Definition>) -> Vec<(String, Expr)> {
    let mut bindings_vec = Vec::new();
    match s {
        Sexp::List(bindings) => {
            for binding in bindings.iter() {
                let current_binding = match binding {
                    Sexp::List(binding) => {
                        match &binding[..] {
                            [Sexp::Atom(S(id)), subexp] => {
                              match id.as_ref() {
                                "true" | "false" | "let" | "block" | "loop" | "break" | "set!" | "input" | "if" => panic!("keyword"),
                                _ => vec![(id.to_string(), parse_expr(subexp, &defs))],
                              }
                            }
                            _ => panic!("Invalid"),  
                        }
                    }  
                    _ => panic!("Invalid"),
                };
                bindings_vec.extend(current_binding);
                println!("bindings_vec {:?}", bindings_vec);
            }
            bindings_vec
        }
        _ => panic!("Invalid"),
    }

}

fn parse_def(s: &Sexp, defined_function_names: &mut HashSet<String>,defs: &Vec<Definition>) -> Definition {
  match s {
    Sexp::List(vec) => match &vec[..] {
      [Sexp::Atom(S(fun_keyword)), Sexp::List(fun_def), fun_body_sexp] 
        if fun_keyword == "fun" => match &fun_def[..] {
          [Sexp::Atom(S(fun_name)), args @ ..] => {
            if defined_function_names.contains(fun_name) {
              panic!("Multiple functions are defined with the same name");
            }
            let mut arg_names = HashSet::new();
            let mut args_vec = Vec::<String>::new();
            for arg in args.iter() {
              if let Sexp::Atom(S(arg_name)) = arg {
                println!("arg in args - {}", arg_name);
                if arg_names.contains(arg_name) {
                  panic!("A function's parameter list has a duplicate name");
                }
                arg_names.insert(arg_name);
                args_vec.push(arg_name.to_string());
              }
            }
          
            defined_function_names.insert(fun_name.to_string());
            return Definition {
              fun_name: fun_name.to_string(),
              args: args_vec, 
              fun_body: parse_expr(fun_body_sexp, &defs),
            };
          }, 
        _ => panic!("Function definition not valid"),
      },
      _ => panic!("Program should have fun_keyword, fun_defs_list and fun_body"),
    }, 
    _ => panic!("Program needs to be a list"),
  }
}

fn is_def(s: &Sexp) -> bool {
  match s {
    Sexp::List(vec) => match &vec[..] {
      [Sexp::Atom(S(fun_keyword)), Sexp::List(_), _] if fun_keyword == "fun" => true, 
      _ => false,
    }, 
    _ => false
  }
}

fn parse_prog(s: &Sexp) -> Program {
  let mut defined_function_names : HashSet<String> = HashSet::new();
  match s {
    Sexp::List(vec) => {
      let mut defs: Vec<Definition> = vec![];
      for def_or_expr in vec {
        if is_def(def_or_expr) {
          defs.push(parse_def(def_or_expr, &mut defined_function_names, &defs));
        } else {
          return Program {
            defs: defs.clone(),
            main: parse_expr(def_or_expr, &defs),
          };
        }
      }
      panic!("No main found");
    }
    _ => panic!("Program needs to be a list"),
  }
}

fn parse_expr(s: &Sexp,defs: &Vec<Definition>) -> Expr {
    match s {
        Sexp::Atom(I(n)) => {
          match i64::try_from(*n) {
            Ok(num) => {
              if num > I63_MAX || num < I63_MIN {
                panic!("Invalid: num can't fit in 63 bits")
              } else {
                Expr::Number(i64::try_from(num << 1).unwrap())
              }
            },
            Err(_) => panic!("Invalid: num can't fit in 63 bits")
          }
        },
        Sexp::Atom(S(id)) => {
            match id.as_ref() {
                "let" | "add1" | "sub1" => panic!("Invalid"),
                "true" => Expr::True,
                "false" => Expr::False,
                "input" => Expr::Input,
                _ => Expr::Id(id.to_string()),
            }
        },
        Sexp::List(vec) => {
            match &vec[..] {
                [Sexp::Atom(S(op)), e] if op == "add1" => Expr::UnOp(Op1::Add1, Box::new(parse_expr(e, &defs))),
                [Sexp::Atom(S(op)), e] if op == "sub1" => Expr::UnOp(Op1::Sub1, Box::new(parse_expr(e, &defs))),
                [Sexp::Atom(S(op)), e] if op == "isnum" => Expr::UnOp(Op1::IsNum, Box::new(parse_expr(e, &defs))),
                [Sexp::Atom(S(op)), e] if op == "isbool" => Expr::UnOp(Op1::IsBool, Box::new(parse_expr(e, &defs))),
                [Sexp::Atom(S(op)), e] if op == "print" => Expr::UnOp(Op1::Print, Box::new(parse_expr(e, &defs))),
                [Sexp::Atom(S(op)), e] if op == "loop" => Expr::Loop(Box::new(parse_expr(e, &defs))),
                [Sexp::Atom(S(op)), e] if op == "break" => Expr::Break(Box::new(parse_expr(e, &defs))),
                [Sexp::Atom(S(op)), e1, e2] if op == "let" => Expr::Let(parse_bind(e1, &defs), Box::new(parse_expr(e2, &defs))),
                [Sexp::Atom(S(op)), e1, e2] if op == "+" => Expr::BinOp(Op2::Plus, Box::new(parse_expr(e1, &defs)), Box::new(parse_expr(e2, &defs))),
                [Sexp::Atom(S(op)), e1, e2] if op == "-" => Expr::BinOp(Op2::Minus, Box::new(parse_expr(e1, &defs)), Box::new(parse_expr(e2, &defs))),
                [Sexp::Atom(S(op)), e1, e2] if op == "*" => Expr::BinOp(Op2::Times, Box::new(parse_expr(e1, &defs)), Box::new(parse_expr(e2, &defs))),
                [Sexp::Atom(S(op)), e1, e2] if op == "=" => Expr::BinOp(Op2::Equal, Box::new(parse_expr(e1, &defs)), Box::new(parse_expr(e2, &defs))),
                [Sexp::Atom(S(op)), e1, e2] if op == ">" => Expr::BinOp(Op2::Greater, Box::new(parse_expr(e1, &defs)), Box::new(parse_expr(e2, &defs))),
                [Sexp::Atom(S(op)), e1, e2] if op == "<" => Expr::BinOp(Op2::Less, Box::new(parse_expr(e1, &defs)), Box::new(parse_expr(e2, &defs))),
                [Sexp::Atom(S(op)), e1, e2] if op == ">=" => Expr::BinOp(Op2::GreaterEqual, Box::new(parse_expr(e1, &defs)), Box::new(parse_expr(e2, &defs))),
                [Sexp::Atom(S(op)), e1, e2] if op == "<=" => Expr::BinOp(Op2::LessEqual, Box::new(parse_expr(e1, &defs)), Box::new(parse_expr(e2, &defs))),

                [Sexp::Atom(S(op)), cond_exp, then_exp, else_exp] 
                  if op == "if" => Expr::If(Box::new(parse_expr(cond_exp, &defs)), 
                                            Box::new(parse_expr(then_exp, &defs)), 
                                            Box::new(parse_expr(else_exp, &defs))),

                [Sexp::Atom(S(op)), Sexp::Atom(S(id)), exp] if op == "set!" => {
                  match id.as_ref() {
                    "true" | "false" | "let" | "block" | "loop" | "break" | "set!" | "input" | "if" => panic!("keyword"),
                    _ => Expr::Set(id.to_string(), Box::new(parse_expr(exp, &defs))),
                  }
                }
                [Sexp::Atom(S(op)), rest @ ..] if op == "block" => {
                  if rest.len() == 0 {
                    panic!("Invalid syntax");
                  }
                  let mut block_exps = Vec::new();
                  for sub_exp in rest.iter() {
                    block_exps.push(parse_expr(sub_exp, &defs));
                  }
                  Expr::Block(block_exps)
                }
                [Sexp::Atom(S(fun_name)), rest @ ..] 
                  if defs.iter().any(|def| def.fun_name == fun_name.to_string() && def.args.len() == rest.len()) => {
                    Expr::Function(fun_name.to_string(), 
                      rest.iter().map(|arg_val| parse_expr(arg_val, &defs)).collect())
                }
                _ => panic!("Invalid {:?}", vec)
            }
        },
        _ => panic!("Invalid")

    }
}



// -------




fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let in_name = &args[1];
    let out_name = &args[2];

    let mut in_file = File::open(in_name)?;
    let mut contents = String::new();
    in_file.read_to_string(&mut contents)?;

    let prog = "(".to_owned() + &contents + ")";

    let s_exp = match parse(&prog) {
        Ok(s_exp) => s_exp,
        Err(_) => panic!("Invalid")
    };

    println!("s_exp - {}", s_exp);
    let prog = parse_prog(&s_exp);
    println!("parse_prog - {:?}", prog);
    // let expr = parse_expr(&s_exp);
    // let result = format!("expr - {:?}", expr);
    // println!("Parsed expr = {:?}", expr);

    // let result = compile(&expr);
    let result = "mov rax, 1";

    let asm_program = format!(
        "
section .text
global our_code_starts_here
extern snek_print
extern snek_error

throw_error:
mov rdi, 7
push rsp
call snek_error
jmp our_code_starts_here

throw_overflow_error:
mov rdi, 8
push rsp
call snek_error
jmp our_code_starts_here

our_code_starts_here:
  {}
  ret
",
        result
    );

    let mut out_file = File::create(out_name)?;
    out_file.write_all(asm_program.as_bytes())?;

    Ok(())
}
