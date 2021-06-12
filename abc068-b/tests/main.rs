use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc068-b", "");
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
            7
        "#,
        r#"
            4
        "#,
  )
}

#[test]
fn sample2() {
    assert_cmd(
        r#"
            32
        "#,
        r#"
            32
        "#,
  )
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            1
        "#,
        r#"
            1
        "#,
  )
}

#[test]
fn sample4() {
    assert_cmd(
        r#"
            100
        "#,
        r#"
            64
        "#,
  )
}
