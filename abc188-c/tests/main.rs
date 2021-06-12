use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc188-c", "");
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
            2
            1 4 2 5
        "#,
        r#"
            2
        "#,
  )
}

#[test]
fn sample2() {
    assert_cmd(
        r#"
            2
            3 1 5 4
        "#,
        r#"
            1
        "#,
  )
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            4
            6 13 12 5 3 7 10 11 16 9 8 15 2 1 14 4
        "#,
        r#"
            2
        "#,
  )
}
