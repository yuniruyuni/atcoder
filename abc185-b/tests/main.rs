use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc185-b", "");
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
            10 2 20
            9 11
            13 17
        "#,
        r#"
            Yes
        "#,
  )
}

#[test]
fn sample2() {
    assert_cmd(
        r#"
            10 2 20
            9 11
            13 16
        "#,
        r#"
            No
        "#,
  )
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            15 3 30
            5 8
            15 17
            24 27
        "#,
        r#"
            Yes
        "#,
  )
}

#[test]
fn sample4() {
    assert_cmd(
        r#"
            20 1 30
            20 29
        "#,
        r#"
            No
        "#,
  )
}

#[test]
fn sample5() {
    assert_cmd(
        r#"
            20 1 30
            1 10
        "#,
        r#"
            No
        "#,
  )
}
