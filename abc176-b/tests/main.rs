use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc176-b", "");
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
            123456789
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
            0
        "#,
        r#"
            Yes
        "#,
  )
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            31415926535897932384626433832795028841971693993751058209749445923078164062862089986280
        "#,
        r#"
            No
        "#,
  )
}
