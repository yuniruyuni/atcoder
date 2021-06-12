use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc112-c", "");
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
            2 3 5
            2 1 5
            1 2 5
            3 2 5
        "#,
        r#"
            2 2 6
        "#,
  )
}

#[test]
fn sample2() {
    assert_cmd(
        r#"
            2
            0 0 100
            1 1 98
        "#,
        r#"
            0 0 100
        "#,
  )
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            3
            99 1 191
            100 1 192
            99 0 192
        "#,
        r#"
            100 0 193
        "#,
  )
}
