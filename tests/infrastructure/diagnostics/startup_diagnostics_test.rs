#![forbid(unsafe_code)]

use humanizar_nucleo::infrastructure::diagnostics::install_panic_hook;
use std::process::Command;
use uuid::Uuid;

#[test]
fn panic_hook_child() {
    if std::env::var("NUCLEO_PANIC_FIXTURE").as_deref() == Ok("1") {
        install_panic_hook();
        panic!("panic_Bearer_password_sentinel");
    }
}

#[test]
fn production_panic_hook_does_not_print_the_panic_payload() {
    let output = Command::new(std::env::current_exe().expect("test executable"))
        .args(["--exact", "panic_hook_child", "--nocapture"])
        .env("NUCLEO_PANIC_FIXTURE", "1")
        .output()
        .expect("child");
    assert!(!output.status.success());
    let diagnostic = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(diagnostic.contains("falha inesperada no processo"));
    assert!(!diagnostic.contains("panic_Bearer_password_sentinel"));
}

#[test]
fn malformed_dotenv_does_not_print_credentials_or_line_content() {
    let directory = std::env::temp_dir().join(format!(
        "nucleo-startup-fixture-{}",
        Uuid::new_v4().simple()
    ));
    std::fs::create_dir(&directory).expect("fixture directory");
    let dotenv = directory.join(".env");
    std::fs::write(&dotenv, "DB_PASSWORD='dotenv_password_sentinel").expect("malformed fixture");
    let mut command = Command::new(env!("CARGO_BIN_EXE_humanizar-nucleo"));
    command.env_clear().current_dir(&directory);
    for name in ["SystemRoot", "WINDIR"] {
        if let Some(value) = std::env::var_os(name) {
            command.env(name, value);
        }
    }
    let output = command.output().expect("service child");
    std::fs::remove_file(dotenv).expect("cleanup fixture file");
    std::fs::remove_dir(directory).expect("cleanup fixture directory");
    assert!(!output.status.success());
    let diagnostic = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(diagnostic.contains("Falha ao carregar arquivo .env"));
    assert!(!diagnostic.contains("dotenv_password_sentinel"));
}
