// MIT License
//
// Copyright (c) 2023 Sergio Arroutbi
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

const RUSTSPYDER_BINARY: &str = "rustspyder";

#[test]
fn help_check_options_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(RUSTSPYDER_BINARY)?;
    cmd.arg("-h");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("help"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("verbose"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("config"));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(RUSTSPYDER_BINARY));
    Ok(())
}
