use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc072-b", "");
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
            atcoder
        "#,
        r#"
            acdr
        "#,
  )
}

#[test]
fn sample2() {
    assert_cmd(
        r#"
            aaaa
        "#,
        r#"
            aa
        "#,
  )
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            z
        "#,
        r#"
            z
        "#,
  )
}

#[test]
fn sample4() {
    assert_cmd(
        r#"
            fukuokayamaguchi
        "#,
        r#"
            fkoaaauh
        "#,
  )
}
