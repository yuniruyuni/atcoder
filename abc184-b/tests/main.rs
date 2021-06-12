use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc184-b", "");
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
            3 0
            xox
        "#,
        r#"
            0
        "#,
  )
}

#[test]
fn sample2() {
    assert_cmd(
        r#"
            20 199999
            oooooooooxoooooooooo
        "#,
        r#"
            200017
        "#,
  )
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            20 10
            xxxxxxxxxxxxxxxxxxxx
        "#,
        r#"
            0
        "#,
  )
}