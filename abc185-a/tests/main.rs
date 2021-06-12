use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc185-a", "");
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
            5 3 7 11
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
            100 100 1 100
        "#,
        r#"
            1
        "#
    );
}
