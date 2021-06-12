use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc061-b", "");
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
            4 3
            1 2
            2 3
            1 4
        "#,
        r#"
            2
            2
            1
            1
        "#,
  )
}

#[test]
fn sample2() {
    assert_cmd(
        r#"
            2 5
            1 2
            2 1
            1 2
            2 1
            1 2
        "#,
        r#"
            5
            5
        "#,
  )
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            8 8
            1 2
            3 4
            1 5
            2 8
            3 7
            5 2
            4 1
            6 8
        "#,
        r#"
            3
            3
            2
            2
            2
            1
            1
            2
        "#,
  )
}
