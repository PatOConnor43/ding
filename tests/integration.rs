#[cfg(test)]
mod tests {

    use insta_cmd::{Command, assert_cmd_snapshot, get_cargo_bin};
    use std::io::Write;

    #[test]
    fn missing_spec() {
        let mut cmd = Command::new(get_cargo_bin("ding"));
        assert_cmd_snapshot!(cmd);
    }

    #[test]
    fn complete_query_parameter() {
        let mut cmd = Command::new(get_cargo_bin("ding"));
        let cmd = cmd
            .arg("--spec")
            .arg("tests/petstore.yaml")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped());

        let mut child = cmd.spawn().expect("Failed to spawn command");

        // Write to stdin
        if let Some(stdin) = child.stdin.take() {
            let mut stdin = stdin;
            stdin
                .write_all(b"curl -X GET https://localhost:9000/pets")
                .expect("Failed to write to stdin");
        }
        let output = child
            .wait_with_output()
            .expect("Failed to wait for command");
        let output_str = String::from_utf8_lossy(&output.stdout);
        insta::assert_snapshot!(output_str);
    }

    #[test]
    fn alternates_query_parameters() {
        let mut cmd = Command::new(get_cargo_bin("ding"));
        let cmd = cmd
            .arg("--spec")
            .arg("tests/petstore.yaml")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped());

        let mut child = cmd.spawn().expect("Failed to spawn command");

        // Write to stdin
        if let Some(stdin) = child.stdin.take() {
            let mut stdin = stdin;
            stdin
                .write_all(b"curl -X GET https://localhost:9000/pets --data-urlencode 'limit='")
                .expect("Failed to write to stdin");
        }
        let output = child
            .wait_with_output()
            .expect("Failed to wait for command");
        let output_bytes = output.stdout;
        let output_str = String::from_utf8_lossy(&output_bytes);
        insta::assert_snapshot!(output_str);

        let mut child = cmd.spawn().expect("Failed to spawn command");

        // Write to stdin
        if let Some(stdin) = child.stdin.take() {
            let mut stdin = stdin;
            stdin
                .write_all(&output_bytes)
                .expect("Failed to write to stdin");
        }
        let output = child
            .wait_with_output()
            .expect("Failed to wait for command");
        let output_str = String::from_utf8_lossy(&output.stdout);
        insta::assert_snapshot!(output_str);
    }

    #[test]
    fn complete_header_parameter() {
        let mut cmd = Command::new(get_cargo_bin("ding"));
        let cmd = cmd
            .arg("--spec")
            .arg("tests/petstore.yaml")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped());

        let mut child = cmd.spawn().expect("Failed to spawn command");

        // Write to stdin
        if let Some(stdin) = child.stdin.take() {
            let mut stdin = stdin;
            stdin
                .write_all(b"curl -X GET https://localhost:9000/petsHeader")
                .expect("Failed to write to stdin");
        }
        let output = child
            .wait_with_output()
            .expect("Failed to wait for command");
        let output_str = String::from_utf8_lossy(&output.stdout);
        insta::assert_snapshot!(output_str);
    }

    #[test]
    fn alternates_header_parameters() {
        let mut cmd = Command::new(get_cargo_bin("ding"));
        let cmd = cmd
            .arg("--spec")
            .arg("tests/petstore.yaml")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped());

        let mut child = cmd.spawn().expect("Failed to spawn command");

        // Write to stdin
        if let Some(stdin) = child.stdin.take() {
            let mut stdin = stdin;
            stdin
                .write_all(b"curl -X GET https://localhost:9000/petsHeader -H 'limit:'")
                .expect("Failed to write to stdin");
        }
        let output = child
            .wait_with_output()
            .expect("Failed to wait for command");
        let output_bytes = output.stdout;
        let output_str = String::from_utf8_lossy(&output_bytes);
        insta::assert_snapshot!(output_str);

        let mut child = cmd.spawn().expect("Failed to spawn command");

        // Write to stdin
        if let Some(stdin) = child.stdin.take() {
            let mut stdin = stdin;
            stdin
                .write_all(&output_bytes)
                .expect("Failed to write to stdin");
        }
        let output = child
            .wait_with_output()
            .expect("Failed to wait for command");
        let output_str = String::from_utf8_lossy(&output.stdout);
        insta::assert_snapshot!(output_str);
    }

    #[test]
    fn complete_request_body_parameter() {
        let mut cmd = Command::new(get_cargo_bin("ding"));
        let cmd = cmd
            .arg("--spec")
            .arg("tests/petstore.yaml")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped());

        let mut child = cmd.spawn().expect("Failed to spawn command");

        // Write to stdin
        if let Some(stdin) = child.stdin.take() {
            let mut stdin = stdin;
            stdin
                .write_all(b"curl -X POST https://localhost:9000/pets")
                .expect("Failed to write to stdin");
        }
        let output = child
            .wait_with_output()
            .expect("Failed to wait for command");
        let output_str = String::from_utf8_lossy(&output.stdout);
        insta::assert_snapshot!(output_str);
    }

    #[test]
    fn complete_command_with_pipe_end() {
        let mut cmd = Command::new(get_cargo_bin("ding"));
        let cmd = cmd
            .arg("--spec")
            .arg("tests/petstore.yaml")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped());

        let mut child = cmd.spawn().expect("Failed to spawn command");

        // Write to stdin
        if let Some(stdin) = child.stdin.take() {
            let mut stdin = stdin;
            stdin
                .write_all(b"curl -X GET https://localhost:9000/pets | jq .")
                .expect("Failed to write to stdin");
        }
        let output = child
            .wait_with_output()
            .expect("Failed to wait for command");
        let output_str = String::from_utf8_lossy(&output.stdout);
        insta::assert_snapshot!(output_str);
    }

    #[test]
    fn complete_command_with_pipe_start() {
        let mut cmd = Command::new(get_cargo_bin("ding"));
        let cmd = cmd
            .arg("--spec")
            .arg("tests/petstore.yaml")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped());

        let mut child = cmd.spawn().expect("Failed to spawn command");

        // Write to stdin
        if let Some(stdin) = child.stdin.take() {
            let mut stdin = stdin;
            stdin
                .write_all(b"echo -n \"test\" | curl -X GET https://localhost:9000/pets")
                .expect("Failed to write to stdin");
        }
        let output = child
            .wait_with_output()
            .expect("Failed to wait for command");
        let output_str = String::from_utf8_lossy(&output.stdout);
        insta::assert_snapshot!(output_str);
    }

    #[test]
    fn complete_command_with_pipe_start_and_end() {
        let mut cmd = Command::new(get_cargo_bin("ding"));
        let cmd = cmd
            .arg("--spec")
            .arg("tests/petstore.yaml")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped());

        let mut child = cmd.spawn().expect("Failed to spawn command");

        // Write to stdin
        if let Some(stdin) = child.stdin.take() {
            let mut stdin = stdin;
            stdin
                .write_all(b"echo -n \"test\" | curl -X GET https://localhost:9000/pets | jq .")
                .expect("Failed to write to stdin");
        }
        let output = child
            .wait_with_output()
            .expect("Failed to wait for command");
        let output_str = String::from_utf8_lossy(&output.stdout);
        insta::assert_snapshot!(output_str);
    }

    #[test]
    fn json_complete_query_command() {
        let mut cmd = Command::new(get_cargo_bin("ding"));
        let cmd = cmd
            .arg("--spec")
            .arg("tests/petstore.yaml")
            .arg("--json")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped());

        let mut child = cmd.spawn().expect("Failed to spawn command");

        // Write to stdin
        if let Some(stdin) = child.stdin.take() {
            let mut stdin = stdin;
            stdin
                .write_all(b"curl -X GET https://localhost:9000/pets")
                .expect("Failed to write to stdin");
        }
        let output = child
            .wait_with_output()
            .expect("Failed to wait for command");
        let output_str = String::from_utf8_lossy(&output.stdout);
        insta::assert_snapshot!(output_str);
    }

    #[test]
    fn json_alternates_query_command() {
        let mut cmd = Command::new(get_cargo_bin("ding"));
        let cmd = cmd
            .arg("--spec")
            .arg("tests/petstore.yaml")
            .arg("--json")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped());

        let mut child = cmd.spawn().expect("Failed to spawn command");

        // Write to stdin
        if let Some(stdin) = child.stdin.take() {
            let mut stdin = stdin;
            stdin
                .write_all(b"curl -X GET https://localhost:9000/pets --data-urlencode 'limit='")
                .expect("Failed to write to stdin");
        }
        let output = child
            .wait_with_output()
            .expect("Failed to wait for command");
        let output_bytes = output.stdout;
        let output_str = String::from_utf8_lossy(&output_bytes);
        insta::assert_snapshot!(output_str);

        let next_stdin: serde_json::Value =
            serde_json::from_slice(&output_bytes).expect("Failed to parse JSON from output");
        let next_stdin = next_stdin
            .as_object()
            .expect("Expected JSON object")
            .get("stdout")
            .expect("Expected 'stdout' key in JSON")
            .as_str()
            .expect("Expected 'stdout' to be a string");

        let mut child = cmd.spawn().expect("Failed to spawn command");

        // Write to stdin
        if let Some(stdin) = child.stdin.take() {
            let mut stdin = stdin;
            stdin
                .write_all(next_stdin.as_bytes())
                .expect("Failed to write to stdin");
        }
        let output = child
            .wait_with_output()
            .expect("Failed to wait for command");
        let output_str = String::from_utf8_lossy(&output.stdout);
        insta::assert_snapshot!(output_str);
    }

    #[test]
    fn json_complete_header_command() {
        let mut cmd = Command::new(get_cargo_bin("ding"));
        let cmd = cmd
            .arg("--spec")
            .arg("tests/petstore.yaml")
            .arg("--json")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped());

        let mut child = cmd.spawn().expect("Failed to spawn command");

        // Write to stdin
        if let Some(stdin) = child.stdin.take() {
            let mut stdin = stdin;
            stdin
                .write_all(b"curl -X GET https://localhost:9000/petsHeader")
                .expect("Failed to write to stdin");
        }
        let output = child
            .wait_with_output()
            .expect("Failed to wait for command");
        let output_str = String::from_utf8_lossy(&output.stdout);
        insta::assert_snapshot!(output_str);
    }

    #[test]
    fn json_alternates_header_command() {
        let mut cmd = Command::new(get_cargo_bin("ding"));
        let cmd = cmd
            .arg("--spec")
            .arg("tests/petstore.yaml")
            .arg("--json")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped());

        let mut child = cmd.spawn().expect("Failed to spawn command");

        // Write to stdin
        if let Some(stdin) = child.stdin.take() {
            let mut stdin = stdin;
            stdin
                .write_all(b"curl -X GET https://localhost:9000/petsHeader -H 'limit:'")
                .expect("Failed to write to stdin");
        }
        let output = child
            .wait_with_output()
            .expect("Failed to wait for command");
        let output_bytes = output.stdout;
        let output_str = String::from_utf8_lossy(&output_bytes);
        insta::assert_snapshot!(output_str);
        let next_stdin: serde_json::Value =
            serde_json::from_slice(&output_bytes).expect("Failed to parse JSON from output");
        let next_stdin = next_stdin
            .as_object()
            .expect("Expected JSON object")
            .get("stdout")
            .expect("Expected 'stdout' key in JSON")
            .as_str()
            .expect("Expected 'stdout' to be a string");

        let mut child = cmd.spawn().expect("Failed to spawn command");

        // Write to stdin
        if let Some(stdin) = child.stdin.take() {
            let mut stdin = stdin;
            stdin
                .write_all(next_stdin.as_bytes())
                .expect("Failed to write to stdin");
        }
        let output = child
            .wait_with_output()
            .expect("Failed to wait for command");
        let output_str = String::from_utf8_lossy(&output.stdout);
        insta::assert_snapshot!(output_str);
    }

    #[test]
    fn json_complete_request_body_command() {
        let mut cmd = Command::new(get_cargo_bin("ding"));
        let cmd = cmd
            .arg("--spec")
            .arg("tests/petstore.yaml")
            .arg("--json")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped());

        let mut child = cmd.spawn().expect("Failed to spawn command");

        // Write to stdin
        if let Some(stdin) = child.stdin.take() {
            let mut stdin = stdin;
            stdin
                .write_all(b"curl -X POST https://localhost:9000/pets")
                .expect("Failed to write to stdin");
        }
        let output = child
            .wait_with_output()
            .expect("Failed to wait for command");
        let output_str = String::from_utf8_lossy(&output.stdout);
        insta::assert_snapshot!(output_str);
    }

    #[test]
    fn json_complete_query_parameter_with_path_parameter() {
        let mut cmd = Command::new(get_cargo_bin("ding"));
        let cmd = cmd
            .arg("--spec")
            .arg("tests/petstore.yaml")
            .arg("--json")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped());

        let mut child = cmd.spawn().expect("Failed to spawn command");

        // Write to stdin
        if let Some(stdin) = child.stdin.take() {
            let mut stdin = stdin;
            stdin
                .write_all(b"curl -X GET https://localhost:9000/pets/123")
                .expect("Failed to write to stdin");
        }
        let output = child
            .wait_with_output()
            .expect("Failed to wait for command");
        let output_str = String::from_utf8_lossy(&output.stdout);
        insta::assert_snapshot!(output_str);
    }

    #[test]
    fn json_complete_query_parameter_with_path_parameter_nested() {
        let mut cmd = Command::new(get_cargo_bin("ding"));
        let cmd = cmd
            .arg("--spec")
            .arg("tests/petstore.yaml")
            .arg("--json")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped());

        let mut child = cmd.spawn().expect("Failed to spawn command");

        // Write to stdin
        if let Some(stdin) = child.stdin.take() {
            let mut stdin = stdin;
            stdin
                .write_all(b"curl -X GET https://localhost:9000/pets/123/owner")
                .expect("Failed to write to stdin");
        }
        let output = child
            .wait_with_output()
            .expect("Failed to wait for command");
        let output_str = String::from_utf8_lossy(&output.stdout);
        insta::assert_snapshot!(output_str);
    }

    #[test]
    fn complete_query_parameter_command_with_prefix() {
        let mut cmd = Command::new(get_cargo_bin("ding"));
        let cmd = cmd
            .arg("--spec")
            .arg("tests/petstore.yaml")
            .arg("--path-prefix")
            .arg("/api/v1")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped());

        let mut child = cmd.spawn().expect("Failed to spawn command");

        // Write to stdin
        if let Some(stdin) = child.stdin.take() {
            let mut stdin = stdin;
            stdin
                .write_all(b"curl -X GET https://localhost:9000/api/v1/pets/123/owner")
                .expect("Failed to write to stdin");
        }
        let output = child
            .wait_with_output()
            .expect("Failed to wait for command");
        let output_str = String::from_utf8_lossy(&output.stdout);
        insta::assert_snapshot!(output_str);
    }

    #[test]
    fn complete_query_parameter_command_without_leading_slash_prefix() {
        let mut cmd = Command::new(get_cargo_bin("ding"));
        let cmd = cmd
            .arg("--spec")
            .arg("tests/petstore.yaml")
            .arg("--path-prefix")
            .arg("api/v1")
            .stdin(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped());

        let mut child = cmd.spawn().expect("Failed to spawn command");

        if let Some(stdin) = child.stdin.take() {
            let mut stdin = stdin;
            stdin
                .write_all(b"curl -X GET https://localhost:9000/api/v1/pets/123/owner")
                .expect("Failed to write to stdin");
        }
        let output = child
            .wait_with_output()
            .expect("Failed to wait for command");
        let output_str = String::from_utf8_lossy(&output.stdout);
        insta::assert_snapshot!(output_str);

        let stderr_str = String::from_utf8_lossy(&output.stderr);
        insta::assert_snapshot!(stderr_str);
    }

    #[test]
    fn complete_query_parameter_command_with_trailing_slash_prefix() {
        let mut cmd = Command::new(get_cargo_bin("ding"));
        let cmd = cmd
            .arg("--spec")
            .arg("tests/petstore.yaml")
            .arg("--path-prefix")
            .arg("/api/v1/")
            .stdin(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped());

        let mut child = cmd.spawn().expect("Failed to spawn command");

        if let Some(stdin) = child.stdin.take() {
            let mut stdin = stdin;
            stdin
                .write_all(b"curl -X GET https://localhost:9000/api/v1/pets/123/owner")
                .expect("Failed to write to stdin");
        }
        let output = child
            .wait_with_output()
            .expect("Failed to wait for command");
        let output_str = String::from_utf8_lossy(&output.stdout);
        insta::assert_snapshot!(output_str);

        let stderr_str = String::from_utf8_lossy(&output.stderr);
        insta::assert_snapshot!(stderr_str);
    }
}
