use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc064-a", "");
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
            4 3 2
        "#,
        r#"
            YES
        "#,
  )
}

#[test]
fn sample2() {
    assert_cmd(
        r#"
            2 3 4
        "#,
        r#"
            NO
        "#,
  )
}
