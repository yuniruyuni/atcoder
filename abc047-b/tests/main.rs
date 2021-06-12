use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc047-b", "");
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
            5 4 2
            2 1 1
            3 3 4
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
            5 4 3
            2 1 1
            3 3 4
            1 4 2
        "#,
        r#"
            0
        "#,
  )
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            10 10 5
            1 6 1
            4 1 3
            6 9 4
            9 4 2
            3 1 3
        "#,
        r#"
            64
        "#,
  )
}
