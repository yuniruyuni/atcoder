use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./arc082-a", "");
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
            7
            3 1 4 1 5 9 2
        "#,
        r#"
            4
        "#,
  )
}

#[test]
fn sample2() {
    assert_cmd(
        r#"
            10
            0 1 2 3 4 5 6 7 8 9
        "#,
        r#"
            3
        "#,
  )
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            1
            99999
        "#,
        r#"
            1
        "#,
  )
}
