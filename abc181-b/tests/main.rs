use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc181-b", "");
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
            1 3
            3 5
        "#,
        r#"
            18
        "#
    );
}


#[test]
fn sample2() {
    assert_cmd(
        r#"
            3
            11 13
            17 47
            359 44683
        "#,
        r#"
            998244353
        "#
    );
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            1
            1 1000000
        "#,
        r#"
            500000500000
        "#
    );
}
