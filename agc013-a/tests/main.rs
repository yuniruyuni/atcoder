use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./agc013-a", "");
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
            6
            1 2 3 2 2 1
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
            9
            1 2 1 2 1 2 1 2 1
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
            7
            1 2 3 2 1 999999999 1000000000
        "#,
        r#"
            3
        "#,
  )
}
