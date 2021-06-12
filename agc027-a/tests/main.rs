use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./agc027-a", "");
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
            3 70
            20 30 10
        "#,
        r#"
            2
        "#,
  )
}

#[test]
fn sample2() {
    assert_cmd(
        r#"
            3 10
            20 30 10
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
            4 1111
            1 10 100 1000
        "#,
        r#"
            4
        "#,
  )
}

#[test]
fn sample4() {
    assert_cmd(
        r#"
            2 10
            20 20
        "#,
        r#"
            0
        "#,
  )
}
