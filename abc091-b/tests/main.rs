use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc091-b", "");
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
            apple
            orange
            apple
            1
            grape
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
            3
            apple
            orange
            apple
            5
            apple
            apple
            apple
            apple
            apple
        "#,
        r#"
            1
        "#,
  )
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            1
            voldemort
            10
            voldemort
            voldemort
            voldemort
            voldemort
            voldemort
            voldemort
            voldemort
            voldemort
            voldemort
            voldemort
        "#,
        r#"
            0
        "#,
  )
}

#[test]
fn sample4() {
    assert_cmd(
        r#"
            6
            red
            red
            blue
            yellow
            yellow
            red
            5
            red
            red
            yellow
            green
            blue
        "#,
        r#"
            1
        "#,
  )
}
