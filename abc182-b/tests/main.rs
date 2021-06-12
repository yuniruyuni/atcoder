use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc182-b", "");
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
            3
            3 12 7
        "#,
        r#"
            3
        "#,
  )
}

#[test]
fn sample2() {
    assert_cmd(
        r#"
            5
            8 9 18 90 72
        "#,
        r#"
            2
        "#,
  )
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            5
            1000 1000 1000 1000 1000
        "#,
        r#"
            2
        "#,
  )
}
