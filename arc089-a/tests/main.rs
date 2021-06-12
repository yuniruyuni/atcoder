use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./arc089-a", "");
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
            3 1 2
            6 1 1
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
            1
            2 100 100
        "#,
        r#"
            No
        "#,
  )
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            2
            5 1 1
            100 1 1
        "#,
        r#"
            No
        "#,
  )
}
