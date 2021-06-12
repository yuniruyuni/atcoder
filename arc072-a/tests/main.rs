use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./arc072-a", "");
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
            4
            1 -3 1 0
        "#,
        r#"
            4
        "#,
  )
}

#[test]
fn sample2() {
    assert_cmd(
        r#"
            5
            3 -6 4 -5 7
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
            6
            -1 4 3 2 -5 4
        "#,
        r#"
            8
        "#,
  )
}

#[test]
fn sample4() {
    assert_cmd(
        r#"
            1
            -1
        "#,
        r#"
            0
        "#,
  )
}

#[test]
fn sample5() {
    assert_cmd(
        r#"
            1
            1
        "#,
        r#"
            0
        "#,
  )
}

#[test]
fn sample6() {
    assert_cmd(
        r#"
            2
            1
            -2
        "#,
        r#"
            0
        "#,
  )
}

#[test]
fn sample7() {
    assert_cmd(
        r#"
            2
            -1
            2
        "#,
        r#"
            0
        "#,
  )
}

#[test]
fn sample8() {
    assert_cmd(
        r#"
            1
            0
        "#,
        r#"
            1
        "#,
  )
}

#[test]
fn sample9() {

    assert_cmd(
        r#"
            3
            1
            2
            3
        "#,
        r#"
            4
        "#,
  )
}
