use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc083-b", "");
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
            20 2 5
        "#,
        r#"
            84
        "#,
  )
}

#[test]
fn sample2() {
    assert_cmd(
        r#"
            10 1 2
        "#,
        r#"
            13
        "#,
  )
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            100 4 16
        "#,
        r#"
            4554
        "#,
  )
}
