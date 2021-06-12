use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc168-c", "");
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
            3 4 9 0
        "#,
        r#"
            5
        "#
    );
}

#[test]
fn sample2() {
    assert_cmd(
        r#"
            3 4 10 40
        "#,
        r#"
            4.564257194330056
        "#
    );
}
