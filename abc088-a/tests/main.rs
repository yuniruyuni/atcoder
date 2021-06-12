use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc088-a", "");
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
            2018
            218
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
            2763
            0
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
            37
            514
        "#,
        r#"
            Yes
        "#,
  )
}
