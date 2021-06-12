use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc113-b", "");
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
            12 5
            1000 2000
        "#,
        r#"
            1
        "#,
  )
}

#[test]
fn sample2() {
    assert_cmd(
        r#"
            3
            21 -11
            81234 94124 52141
        "#,
        r#"
            3
        "#,
  )
}
