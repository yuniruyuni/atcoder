use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc188-b", "");
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
            2
            -3 6
            4 2
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
            2
            4 5
            -1 -3
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
            3
            1 3 5
            3 -6 3
        "#,
        r#"
            Yes
        "#,
  )
}
