use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./agc010-a", "");
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
            1 2 3
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
            5
            1 2 3 4 5
        "#,
        r#"
            NO
        "#,
  )
}
