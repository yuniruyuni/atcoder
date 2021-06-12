use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./agc011-a", "");
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
            5 3 5
            1
            2
            3
            6
            12
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
            6 3 3
            7
            6
            2
            8
            10
            6
        "#,
        r#"
            3
        "#,
  )
}
