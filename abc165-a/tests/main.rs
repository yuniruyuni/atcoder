use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc165-a", "");
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
            500 600
        "#,
        r#"
            OK
        "#
    );
}

#[test]
fn sample2() {
    assert_cmd(
        r#"
            4
            5 7
        "#,
        r#"
            NG
        "#
    );
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            1
            11 11
        "#,
        r#"
            OK
        "#
    );
}

#[test]
fn test1() {
    assert_cmd(
        r#"
            3
            4 5
        "#,
        r#"
            NG
        "#
    );
}

#[test]
fn test2() {
    assert_cmd(
        r#"
            3
            4 6
        "#,
        r#"
            OK
        "#
    );
}

#[test]
fn test3() {
    assert_cmd(
        r#"
            3
            3 4
        "#,
        r#"
            OK
        "#
    );
}
