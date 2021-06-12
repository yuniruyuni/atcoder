use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc156-b", "");
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
            11 2
        "#,
        r#"
            4
        "#,
  )
}

#[test]
fn sample2() {
    assert_cmd(
        r#"
            1010101 10
        "#,
        r#"
            7
        "#,
  )
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            314159265 3
        "#,
        r#"
            18
        "#,
  )
}
