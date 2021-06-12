use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc175-b", "");
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
            5
            4 4 9 7 5
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
            6
            4 5 4 3 3 5
        "#,
        r#"
            8
        "#,
  )
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            10
            9 4 6 1 9 6 10 6 6 8
        "#,
        r#"
            39
        "#,
  )
}

#[test]
fn sample4() {
    assert_cmd(
        r#"
            2
            1 1
        "#,
        r#"
            0
        "#,
  )
}
