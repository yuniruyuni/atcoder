use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc088-c", "");
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
            1 0 1
            2 1 2
            1 0 1
        "#,
        r#"
            Yes
        "#,
  )
}

#[test]
fn sample2() {
    assert_cmd(
        r#"
            2 2 2
            2 1 2
            2 2 2
        "#,
        r#"
            No
        "#,
  )
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            0 8 8
            0 8 8
            0 8 8
        "#,
        r#"
            Yes
        "#,
  )
}

#[test]
fn sample4() {
    assert_cmd(
        r#"
            1 8 6
            2 9 7
            0 7 7
        "#,
        r#"
            No
        "#,
  )
}
