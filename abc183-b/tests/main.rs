use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc183-b", "");
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
            1 1 7 2
        "#,
        r#"
            3
        "#
    );
}

#[test]
fn sample2() {
    assert_cmd(
        r#"
            1 1 3 2
        "#,
        r#"
            1.6666666666666667
        "#
    );
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            -9 99 -999 9999
        "#,
        r#"
            -18.705882352941178
        "#
    );
}
