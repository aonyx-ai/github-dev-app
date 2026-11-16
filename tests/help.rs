use std::process::Command;

use anyhow::{Context, Error};
use assert_cmd::prelude::*;

#[test]
fn prints_help() -> Result<(), Error> {
    let mut command = Command::cargo_bin("github-dev-app").context("failed to find the binary")?;

    command.arg("--help");

    command
        .assert()
        .success()
        .stdout(predicates::str::contains("Usage: github-dev-app"));

    Ok(())
}
