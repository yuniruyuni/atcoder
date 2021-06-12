use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc164-a", "");
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
            4 5
        "#,
        r#"
            unsafe
        "#
    );
}

#[test]
fn sample2() {
    assert_cmd(
        r#"
            100 2
        "#,
        r#"
            safe
        "#
    );
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            10 10
        "#,
        r#"
            unsafe
        "#
    );
}
