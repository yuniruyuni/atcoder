use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc095-b", "");
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
            3 1000
            120
            100
            140
        "#,
        r#"
            9
        "#,
  )
}

#[test]
fn sample2() {
    assert_cmd(
        r#"
            4 360
            90
            90
            90
            90
        "#,
        r#"
            4
        "#,
  )
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            5 3000
            150
            130
            150
            130
            110
        "#,
        r#"
            26
        "#,
  )
}
