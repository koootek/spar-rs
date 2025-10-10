#![allow(dead_code, unused_variables, non_camel_case_types, non_snake_case)]

#[derive(PartialEq)]
pub enum LogLevel {
    INFO,
    WARNING,
    ERROR,
    NO_LOGS,
}

macro_rules! log {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {
        let s = format!($fmt, $($arg),*);
        println!("[INFO] {s}");
    };
    ($level:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {
        let s = format!($fmt, $($arg),*);
        match $level {
            $crate::cnot::LogLevel::INFO    => println!("[INFO] {s}"),
            $crate::cnot::LogLevel::WARNING => println!("[WARN] {s}"),
            $crate::cnot::LogLevel::ERROR   => eprintln!("[ERROR] {s}"),
            _ => {},
        }
    };
}

pub(crate) use log;

pub enum RustEdition {
    R2024,
    R2021,
    R2018,
}

impl std::fmt::Display for RustEdition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match self {
            Self::R2024 => "2024",
            Self::R2021 => "2021",
            Self::R2018 => "2018",
        })
    }
}

macro_rules! unwrap_bool {
    ($fn:expr) => {
        match $fn {
            Ok(value) => value,
            Err(_) => return true,
        }
    };
    ($fn:expr, $errval:expr) => {
        match $fn {
            Ok(value) => value,
            Err(_) => return $errval,
        }
    };
}

fn needs_rebuild(output_path: &str, source_paths: &[&str]) -> bool {
    let output_meta = unwrap_bool!(std::fs::metadata(output_path));

    for source_path in source_paths {
        let source_meta = unwrap_bool!(std::fs::metadata(source_path));
        let output_time = unwrap_bool!(output_meta.modified());
        let source_time = unwrap_bool!(source_meta.modified());
        if output_time < source_time {
            return true;
        }
    }
    false
}

/// Rebuilds the program with predefined edition (R2024) and O3 optimizations.
///
/// First arg in `proc_args` must be the path to the executable.
pub fn rebuild<T>(proc_args: &mut dyn Iterator<Item = String>, main_path: &str, extra_sources: &[T])
where
    T: AsRef<str>,
{
    rebuild_edition(proc_args, RustEdition::R2024, main_path, extra_sources);
}

/// Rebuilds the program with O3 optimizations and a custom edition.
///
/// First arg in `proc_args` must be the path to the executable.
pub fn rebuild_edition<T>(
    proc_args: &mut dyn Iterator<Item = String>,
    edition: RustEdition,
    main_path: &str,
    extra_sources: &[T],
) where
    T: AsRef<str>,
{
    rebuild_edition_args(
        proc_args,
        edition,
        main_path,
        extra_sources,
        &[("-O", None)],
    );
}

/// Rebuilds the program with no additional flags and a custom edition.
///
/// First arg in `proc_args` must be the path to the executable.
pub fn rebuild_edition_args<T>(
    proc_args: &mut dyn Iterator<Item = String>,
    edition: RustEdition,
    main_path: &str,
    extra_sources: &[T],
    rustc_args: &[(&str, Option<&str>)],
) where
    T: AsRef<str>,
{
    let self_path = match proc_args.next() {
        Some(self_path) => self_path,
        None => return,
    };
    let mut source_paths = vec![main_path];
    source_paths.append(&mut extra_sources.iter().map(|path| path.as_ref()).collect());
    if !needs_rebuild(&self_path, &source_paths) {
        return;
    }

    let mut args = vec![];
    for (arg, value) in rustc_args {
        args.push(arg);
        if let Some(value) = value {
            args.push(value);
        }
    }

    let status = std::process::Command::new("rustc")
        .args(args)
        .args([
            "--edition",
            &edition.to_string(),
            "-o",
            &self_path,
            main_path,
        ])
        .status()
        .expect("failed to rebuild");

    if !status.success() {
        log!(LogLevel::ERROR, "Build failed");
        std::process::exit(1);
    }

    log!(LogLevel::INFO, "Build successful");
    std::process::Command::new(&self_path)
        .args(proc_args)
        .spawn()
        .expect("program failed to run")
        .wait()
        .expect("program did not run");
    std::process::exit(0);
}

/// Generates `rust-project.json` to fix rust-analyzer not working on standalone files.
pub fn generate_project(root_file: &str, edition: RustEdition) -> std::io::Result<()> {
    if std::fs::exists("rust-project.json")? {
        return Ok(());
    }

    let sysroot_path = std::process::Command::new("rustc")
        .args(["--print", "sysroot"])
        .output()
        .expect("failed to get sysroot");
    if !sysroot_path.status.success() {
        eprintln!("Failed to get sysroot path");
        return Ok(());
    }
    let sysroot_path = String::from_utf8(sysroot_path.stdout).unwrap();
    let mut sysroot_path = sysroot_path.lines();

    std::fs::write(
        "rust-project.json",
        &format!(
            r#"{{
"sysroot_src": "{}",
"crates": [
    {{
        "root_module": "{}",
        "edition": "{}",
        "deps": []
    }}
]
}}"#,
            format!(
                "{}/lib/rustlib/src/rust/library",
                sysroot_path.next().unwrap()
            ),
            root_file,
            edition
        ),
    )?;
    Ok(())
}
