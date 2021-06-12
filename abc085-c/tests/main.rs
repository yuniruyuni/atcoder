use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc085-c", "");
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
            9 45000
        "#,
        r#"
            0 9 0
        "#,
  )
}

#[test]
fn sample2() {
    assert_cmd(
        r#"
            20 196000
        "#,
        r#"
            -1 -1 -1
        "#,
  )
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            1000 1234000
        "#,
        r#"
            2 54 944
        "#,
  )
}

#[test]
fn sample4() {
    assert_cmd(
        r#"
            2000 20000000
        "#,
        r#"
            2000 0 0
        "#,
  )
}
