use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc133-b", "");
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
            3 2
            1 2
            5 5
            -2 8
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
            3 4
            -3 7 8 2
            -12 1 10 2
            -2 8 9 3
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
            5 1
            1
            2
            3
            4
            5
        "#,
        r#"
            10
        "#,
  )
}
