use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc180-b", "");
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
            2 -1
        "#,
        r#"
            3
            2.23606797749979
            2
        "#
    );
}

#[test]
fn sample2() {
    assert_cmd(
        r#"
            10
            3 -1 -4 1 -5 9 2 -6 5 -3
        "#,
        r#"
            39
            14.38749456993816
            9
        "#
    );
}
