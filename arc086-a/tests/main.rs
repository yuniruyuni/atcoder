use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./arc086-a", "");
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
            5 2
            1 1 2 2 5
        "#,
        r#"
            1
        "#,
  )
}

#[test]
fn sample2() {
    assert_cmd(
        r#"
            4 4
            1 1 2 2
        "#,
        r#"
            0
        "#,
  )
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            10 3
            5 1 3 2 4 1 1 2 3 4
        "#,
        r#"
            3
        "#,
  )
}
