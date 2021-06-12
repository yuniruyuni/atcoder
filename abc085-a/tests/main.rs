use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc085-a", "");
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
            2017/01/07
        "#,
        r#"
            2018/01/07
        "#,
  )
}

#[test]
fn sample2() {
    assert_cmd(
        r#"
            2017/01/31
        "#,
        r#"
            2018/01/31
        "#,
  )
}
