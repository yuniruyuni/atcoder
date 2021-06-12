use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc082-b", "");
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
            yx
            axy
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
            ratcode
            atlas
        "#,
        r#"
            Yes
        "#,
  )
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            cd
            abc
        "#,
        r#"
            No
        "#,
  )
}

#[test]
fn sample4() {
    assert_cmd(
        r#"
            w
            ww
        "#,
        r#"
            Yes
        "#,
  )
}

#[test]
fn sample5() {
    assert_cmd(
        r#"
            zzz
            zzz
        "#,
        r#"
            No
        "#,
  )
}
