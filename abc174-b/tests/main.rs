use cli_test_dir::*;
use unindent::unindent;

fn assert_cmd(stdin: &str, stdout: &str) {
    let testdir = TestDir::new("./abc174-b", "");
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
            4 5
            0 5
            -2 4
            3 4
            4 -4
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
            12 3
            1 1
            1 1
            1 1
            1 1
            1 2
            1 3
            2 1
            2 2
            2 3
            3 1
            3 2
            3 3
        "#,
        r#"
            7
        "#
    );
}

#[test]
fn sample3() {
    assert_cmd(
        r#"
            20 100000
            14309 -32939
            -56855 100340
            151364 25430
            103789 -113141
            147404 -136977
            -37006 -30929
            188810 -49557
            13419 70401
            -88280 165170
            -196399 137941
            -176527 -61904
            46659 115261
            -153551 114185
            98784 -6820
            94111 -86268
            -30401 61477
            -55056 7872
            5901 -163796
            138819 -185986
            -69848 -96669
        "#,
        r#"
            6
        "#
    );
}
