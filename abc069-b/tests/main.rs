use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc069-b", "");
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
            internationalization
        "#,
        r#"
            i18n
        "#,
  )
}

#[test]
fn sample2() {
    assert_cmd(
        r#"
            smiles
        "#,
        r#"
            s4s
        "#,
  )
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            xyz
        "#,
        r#"
            x1z
        "#,
  )
}
