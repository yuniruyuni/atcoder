use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc171-b", "");
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
            5 3
            50 100 80 120 80
        "#,
        r#"
            210
        "#,
  )
}

#[test]
fn sample2() {
    assert_cmd(
        r#"
            1 1
            1000
        "#,
        r#"
            1000
        "#,
  )
}
