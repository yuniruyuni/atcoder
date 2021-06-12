use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./arc096-a", "");
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
            1500 2000 1600 3 2
        "#,
        r#"
            7900
        "#,
  )
}

#[test]
fn sample2() {
    assert_cmd(
        r#"
            1500 2000 1900 3 2
        "#,
        r#"
            8500
        "#,
  )
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            1500 2000 500 90000 100000
        "#,
        r#"
            100000000
        "#,
  )
}
