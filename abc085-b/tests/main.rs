use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc085-b", "");
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
            4
            10
            8
            8
            6
        "#,
        r#"
            3
        "#,
  )
}

#[test]
fn sample2() {
    assert_cmd(
        r#"
            3
            15
            15
            15
        "#,
        r#"
            1
        "#,
  )
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            7
            50
            30
            50
            100
            50
            80
            30
        "#,
        r#"
            4
        "#,
  )
}
