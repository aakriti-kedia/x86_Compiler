use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub(crate) enum TestKind {
    Success,
    RuntimeError,
    StaticError,
}

#[macro_export]
macro_rules! success_tests {
    ($($tt:tt)*) => { $crate::tests!(Success => $($tt)*); }
}

#[macro_export]
macro_rules! runtime_error_tests {
    ($($tt:tt)*) => { $crate::tests!(RuntimeError => $($tt)*); }
}

#[macro_export]
macro_rules! static_error_tests {
    ($($tt:tt)*) => { $crate::tests!(StaticError => $($tt)*); }
}

#[macro_export]
macro_rules! tests {
    ($kind:ident =>
        $(
            {
                name: $name:ident,
                file: $file:literal,
                $(input: $input:literal,)?
                expected: $expected:literal $(,)?
                $(" $(tt:$tt)* ")?
            }
        ),*
        $(,)?
    ) => {
        $(
            #[test]
            fn $name() {
                #[allow(unused_assignments, unused_mut)]
                let mut input = None;
                $(input = Some($input);)?
                let kind = $crate::infra::TestKind::$kind;
                $crate::infra::run_test(stringify!($name), $file, input, $expected, kind);
            }
        )*
    };
}

pub(crate) fn run_test(
    name: &str,
    file: &str,
    input: Option<&str>,
    expected: &str,
    kind: TestKind,
) {
    let file = Path::new("tests").join(file);
    match kind {
        TestKind::Success => run_success_test(name, &file, expected, input),
        TestKind::RuntimeError => run_runtime_error_test(name, &file, expected, input),
        TestKind::StaticError => run_static_error_test(name, &file, expected),
    }
}

fn run_success_test(name: &str, file: &Path, expected: &str, input: Option<&str>) {
    if let Err(err) = compile(name, file) {
        panic!("expected a successful compilation, but got an error: `{err}`");
    }
    match run(name, input) {
        Err(err) => {
            panic!("expected a successful execution, but got an error: `{err}`");
        }
        Ok(actual_output) => {
            diff(expected, actual_output);
        }
    }
}

fn run_runtime_error_test(name: &str, file: &Path, expected: &str, input: Option<&str>) {
    if let Err(err) = compile(name, file) {
        panic!("expected a successful compilation, but got an error: `{err}`");
    }
    match run(name, input) {
        Ok(out) => {
            panic!("expected a runtime error, but program executed succesfully - expected error: `{expected}`, output: `{out}`");
        }
        Err(err) => check_error_msg(&err, expected),
    }
}

fn run_static_error_test(name: &str, file: &Path, expected: &str) {
    match compile(name, file) {
        Ok(()) => {
            panic!(
                "expected a static error, but compilation succeeded - expected error: `{expected}`"
            )
        }
        Err(err) => check_error_msg(&err, expected),
    }
}

fn compile(name: &str, file: &Path) -> Result<(), String> {
    // Run the compiler
    let compiler: PathBuf = ["target", "debug", env!("CARGO_PKG_NAME")].iter().collect();
    let output = Command::new(&compiler)
        .arg(file)
        .arg(&mk_path(name, Ext::Asm))
        .output()
        .expect("could not run the compiler");
    if !output.status.success() {
        return Err(String::from_utf8(output.stderr).unwrap());
    }

    // Assemble and link
    let output = Command::new("make")
        .arg(&mk_path(name, Ext::Run))
        .output()
        .expect("could not run make");
    assert!(output.status.success(), "linking failed");

    Ok(())
}

fn run(name: &str, input: Option<&str>) -> Result<String, String> {
    let mut cmd = Command::new(&mk_path(name, Ext::Run));
    if let Some(input) = input {
        cmd.arg(input);
    }
    let output = cmd.output().unwrap();
    if output.status.success() {
        Ok(String::from_utf8(output.stdout).unwrap().trim().to_string())
    } else {
        Err(String::from_utf8(output.stderr).unwrap().trim().to_string())
    }
}

fn check_error_msg(found: &str, expected: &str) {
    let lower_found = found.trim().to_lowercase();
    let lower_expected = expected.trim().to_lowercase();
    assert!(
        lower_found.contains(&lower_expected),
        "the reported error message does not contain the expected subtring - found: `{found}`, expected: `{expected}`",
    );
}

fn diff(expected: &str, found: String) {
    let expected = expected.trim();

    let expected_lines: Vec<&str> = expected.lines().collect();
    let actual_lines: Vec<&str> = found.lines().collect();
    if expected_lines != actual_lines {
        eprintln!(
            "output differed!\n{}",
            prettydiff::diff_lines(&found, expected)
        );
        panic!("test failed");
    }
}

fn mk_path(name: &str, ext: Ext) -> PathBuf {
    Path::new("tests").join(format!("{name}.{ext}"))
}

#[derive(Copy, Clone)]
enum Ext {
    Asm,
    Run,
}

impl std::fmt::Display for Ext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ext::Asm => write!(f, "s"),
            Ext::Run => write!(f, "run"),
        }
    }
}
