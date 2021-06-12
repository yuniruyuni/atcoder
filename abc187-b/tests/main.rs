use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc187-b", "");
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
            3
            0 0
            1 2
            2 1
        "#,
        r#"
            2
        "#,
  )
}

#[test]
fn sample2() {
    assert_cmd(
        r#"
            1
            -691 273
        "#,
        r#"
            0
        "#,
  )
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            10
            -31 -35
            8 -36
            22 64
            5 73
            -14 8
            18 -58
            -41 -85
            1 -88
            -21 -85
            -11 82
        "#,
        r#"
            11
        "#,
  )
}
