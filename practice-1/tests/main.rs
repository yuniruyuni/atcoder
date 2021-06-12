use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./practice-1", "");
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
            1
            2 3
            test
        "#,
        r#"
            6 test
        "#
    );
}


#[test]
fn sample2() {
    assert_cmd(
        r#"
            72
            128 256
            myonmyon
        "#,
        r#"
            456 myonmyon
        "#
    );
}
