use std::{path, process};

#[test]
fn dev_build() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir_path = out_dir.path();

    let app_src_path = path::PathBuf::from("tests/assets/app.rs");
    let build_app_output = build_app(out_dir_path, &app_src_path, false);
    assert!(build_app_output.status.success());

    let app_output = run_app(out_dir_path);
    let expected_stderr = r#"thread 'main' panicked at 'not yet implemented', tests/assets/app.rs:4:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
"#;
    assert!(!app_output.status.success());
    assert_eq!(expected_stderr, String::from_utf8_lossy(&app_output.stderr));
}

#[test]
fn dev_build_with_message() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir_path = out_dir.path();

    let app_src_path = path::PathBuf::from("tests/assets/app_with_message.rs");
    let build_app_output = build_app(out_dir_path, &app_src_path, false);
    assert!(build_app_output.status.success());

    let app_output = run_app(out_dir_path);
    let expected_stderr = r#"thread 'main' panicked at 'today is a rainy day', tests/assets/app_with_message.rs:4:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
"#;
    assert!(!app_output.status.success());
    assert_eq!(expected_stderr, String::from_utf8_lossy(&app_output.stderr));
}

#[test]
fn release_build() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir_path = out_dir.path();

    let app_src_path = path::PathBuf::from("tests/assets/app.rs");
    let build_app_output = build_app(out_dir_path, &app_src_path, true);

    let expected_stderr = r#"error: release blocked
 --> tests/assets/app.rs:4:5
  |
4 |     noship!();
  |     ^^^^^^^^^^
  |
  = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

"#;
    assert!(!build_app_output.status.success());
    assert_eq!(
        expected_stderr,
        String::from_utf8_lossy(&build_app_output.stderr)
    );
}

#[test]
fn release_build_with_message() {
    let out_dir = tempfile::tempdir().unwrap();
    let out_dir_path = out_dir.path();

    let app_src_path = path::PathBuf::from("tests/assets/app_with_message.rs");
    let build_app_output = build_app(out_dir_path, &app_src_path, true);

    let expected_stderr = r#"error: today is a rainy day
 --> tests/assets/app_with_message.rs:4:5
  |
4 |     noship!("today is a rainy day");
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

"#;
    assert!(!build_app_output.status.success());
    assert_eq!(
        expected_stderr,
        String::from_utf8_lossy(&build_app_output.stderr)
    );
}

fn build_libnoship(out_dir_path: &path::Path, release_profile: bool) -> std::path::PathBuf {
    let args = {
        let default_args = [
            "--crate-name",
            "noship",
            "--edition=2018",
            "src/lib.rs",
            "--crate-type",
            "lib",
            "--out-dir",
            out_dir_path.to_str().unwrap(),
        ];
        if release_profile {
            [&default_args[..], &["-O"]].concat()
        } else {
            default_args.to_vec()
        }
    };
    let output = process::Command::new("rustc").args(&args).output().unwrap();
    assert!(output.status.success());

    {
        let mut path = out_dir_path.to_owned();
        path.push("libnoship.rlib");
        path
    }
}

fn build_app(
    out_dir_path: &path::Path,
    src_path: &path::Path,
    release_profile: bool,
) -> process::Output {
    let libnoship_path = build_libnoship(out_dir_path, release_profile);
    let args = {
        let mut args = vec![
            "--crate-name",
            "app",
            "--edition=2018",
            src_path.to_str().unwrap(),
            "--crate-type",
            "bin",
            "--out-dir",
            out_dir_path.to_str().unwrap(),
            "--extern",
        ]
        .iter()
        .map(|&x| x.to_owned())
        .collect::<Vec<_>>();
        args.push(format!("noship={}", libnoship_path.to_str().unwrap()));
        if release_profile {
            args.push("-O".to_owned());
        }
        args
    };
    process::Command::new("rustc").args(&args).output().unwrap()
}

fn run_app(out_dir_path: &path::Path) -> process::Output {
    let app_path = {
        let mut path = out_dir_path.to_owned();
        path.push("app");
        path
    };
    process::Command::new(&*app_path.to_string_lossy())
        .output()
        .unwrap()
}
