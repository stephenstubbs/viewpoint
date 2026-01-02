#![cfg(feature = "integration")]
#![cfg(unix)]

//! Zombie process cleanup tests.
//!
//! These tests verify that browser processes are properly reaped to prevent
//! zombie processes from accumulating.

mod common;

use std::time::Duration;

use common::init_tracing;
use serial_test::serial;
use viewpoint_core::Browser;

/// Helper to count zombie chromium processes for a given PID.
/// Returns the number of defunct chromium processes whose parent is our process.
fn count_zombie_chromium_processes() -> usize {
    use std::process::Command;

    // On Unix, use ps to find zombie (defunct) processes
    // We look for processes in 'Z' state (zombie)
    let output = Command::new("ps")
        .args(["-eo", "pid,ppid,stat,comm"])
        .output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let our_pid = std::process::id();

            stdout
                .lines()
                .filter(|line| {
                    // Look for zombie state ('Z' in stat column) and chromium processes
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 4 {
                        let ppid = parts[1].parse::<u32>().unwrap_or(0);
                        let stat = parts[2];
                        let comm = parts[3];

                        // Check if it's our child, zombie state, and chromium-related
                        ppid == our_pid
                            && stat.starts_with('Z')
                            && (comm.contains("chrom") || comm.contains("Chrom"))
                    } else {
                        false
                    }
                })
                .count()
        }
        Err(_) => 0,
    }
}

/// Test that no zombie processes remain after browser.close().
#[tokio::test]
#[serial]
async fn test_no_zombie_after_close() {
    init_tracing();

    let zombies_before = count_zombie_chromium_processes();

    // Launch and close a browser
    let browser = Browser::launch()
        .headless(true)
        .timeout(Duration::from_secs(30))
        .launch()
        .await
        .expect("Failed to launch browser");

    // Close the browser properly
    browser.close().await.expect("Failed to close browser");

    // Give time for process state to update and all children to be reaped
    tokio::time::sleep(Duration::from_millis(200)).await;

    let zombies_after = count_zombie_chromium_processes();

    assert!(
        zombies_after <= zombies_before,
        "New zombie processes detected after close: before={zombies_before}, after={zombies_after}"
    );
}

/// Test that no zombie processes remain after browser is dropped.
#[tokio::test]
#[serial]
async fn test_no_zombie_after_drop() {
    init_tracing();

    let zombies_before = count_zombie_chromium_processes();

    // Launch a browser and let it drop without explicit close
    {
        let browser = Browser::launch()
            .headless(true)
            .timeout(Duration::from_secs(30))
            .launch()
            .await
            .expect("Failed to launch browser");

        // Just verify it launched
        assert!(browser.is_owned());

        // Browser is dropped here without explicit close
    }

    // Give time for process state to update and all children to be reaped
    tokio::time::sleep(Duration::from_millis(200)).await;

    let zombies_after = count_zombie_chromium_processes();

    assert!(
        zombies_after <= zombies_before,
        "New zombie processes detected after drop: before={zombies_before}, after={zombies_after}"
    );
}

/// Test that no zombie processes remain when browser process dies before close.
#[tokio::test]
#[serial]
async fn test_no_zombie_when_process_dies_before_close() {
    use nix::sys::signal::{Signal, kill};
    use nix::unistd::Pid;

    init_tracing();

    let zombies_before = count_zombie_chromium_processes();

    // Launch a browser
    let browser = Browser::launch()
        .headless(true)
        .timeout(Duration::from_secs(30))
        .launch()
        .await
        .expect("Failed to launch browser");

    // Find the chromium process ID by looking at child processes
    // We'll use ps to find chromium processes with our process as parent
    let our_pid = std::process::id();
    let output = std::process::Command::new("ps")
        .args(["-eo", "pid,ppid,comm"])
        .output()
        .expect("Failed to run ps");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let chromium_pid: Option<i32> = stdout.lines().find_map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let pid = parts[0].parse::<i32>().ok()?;
            let ppid = parts[1].parse::<u32>().ok()?;
            let comm = parts[2];
            if ppid == our_pid && (comm.contains("chrom") || comm.contains("Chrom")) {
                return Some(pid);
            }
        }
        None
    });

    if let Some(pid) = chromium_pid {
        // Kill the chromium process externally
        let _ = kill(Pid::from_raw(pid), Signal::SIGKILL);

        // Give time for the process to die
        tokio::time::sleep(Duration::from_millis(200)).await;
    }

    // Now close the browser (which should reap the already-dead process)
    browser.close().await.expect("Failed to close browser");

    // Give time for process state to update and all children to be reaped
    tokio::time::sleep(Duration::from_millis(200)).await;

    let zombies_after = count_zombie_chromium_processes();

    assert!(
        zombies_after <= zombies_before,
        "New zombie processes detected after close of killed browser: before={zombies_before}, after={zombies_after}"
    );
}
