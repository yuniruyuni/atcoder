use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc186-b", "");
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
            2 3
            2 2 3
            3 2 2
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
            3 3
            99 99 99
            99 0 99
            99 99 99
        "#,
        r#"
            792
        "#,
  )
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            3 2
            4 4
            4 4
            4 4
        "#,
        r#"
            0
        "#,
  )
}
