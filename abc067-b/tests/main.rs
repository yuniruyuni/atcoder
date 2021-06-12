use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc067-b", "");
    let output = testdir
        .cmd()
        .output_with_stdin(unindent(stdin))
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), unindent(stdout));
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample1() {
    assert_cmd(
        r#"
            5 3
            1 2 3 4 5
        "#,
        r#"
            12
        "#,
  )
}

#[test]
fn sample2() {
    assert_cmd(
        r#"
            15 14
            50 26 27 21 41 7 42 35 7 5 5 36 39 1 45
        "#,
        r#"
            386
        "#,
  )
}
