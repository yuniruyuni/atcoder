use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc184-a", "");
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
            1 2
            3 4
        "#,
        r#"
            -2
        "#
    );
}


#[test]
fn sample2() {
    assert_cmd(
        r#"
            0 -1
            1 0
        "#,
        r#"
            1
        "#
    );
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            100 100
            100 100
        "#,
        r#"
            0
        "#
    );
}
