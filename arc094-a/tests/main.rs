use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./arc094-a", "");
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
            2 5 4
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
            2 6 3
        "#,
        r#"
            5
        "#,
  )
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            31 41 5
        "#,
        r#"
            23
        "#,
  )
}
