use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc102-b", "");
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
            4
            1 4 6 3
        "#,
        r#"
            5
        "#,
  )
}

#[test]
fn sample2() {
    assert_cmd(
        r#"
            2
            1000000000 1
        "#,
        r#"
            999999999
        "#,
  )
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            5
            1 1 1 1 1
        "#,
        r#"
            0
        "#,
  )
}
