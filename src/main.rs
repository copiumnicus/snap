use std::fs::{read_dir, read_to_string, remove_file, rename};
use std::io::{self, Write};
use std::path::PathBuf;

fn main() {
    let snapshots_dir = std::env::current_dir()
        .expect("Failed to get current directory")
        .join("snapshots");

    if !snapshots_dir.exists() {
        println!("No snapshots directory found.");
        return;
    }

    // Collect all .new files
    let new_snapshots: Vec<PathBuf> = read_dir(&snapshots_dir)
        .expect("Failed to read snapshots directory")
        .filter_map(|entry| {
            let path = entry.expect("Failed to read directory entry").path();
            if path.extension().and_then(|s| s.to_str()) == Some("new") {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    if new_snapshots.is_empty() {
        println!("No new snapshots to review.");
        return;
    }

    for new_path in new_snapshots {
        let name = new_path.file_stem().unwrap().to_string_lossy().to_string();
        let settled_path = new_path.with_extension("snap");

        let new_content = read_to_string(&new_path).expect("Failed to read new snapshot");

        let is_new_snapshot = !settled_path.exists();
        let settled_content = if !is_new_snapshot {
            Some(read_to_string(&settled_path).expect("Failed to read settled snapshot"))
        } else {
            None
        };

        // Display status and differences
        println!("Snapshot '{}':", name);
        if is_new_snapshot {
            println!("This is a new snapshot.");
            // Print the new snapshot in green
            println!("\x1b[32m{}\x1b[0m", new_content);
        } else {
            println!("Snapshot has changed. Differences:");
            print_differences(&settled_content.as_ref().unwrap(), &new_content);
        }

        // Prompt user to approve, reject, or skip
        loop {
            print!("Approve, reject, or skip snapshot '{}'? (a/r/s): ", name);
            io::stdout().flush().expect("Failed to flush stdout");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");

            match input.trim().to_lowercase().as_str() {
                "a" | "approve" => {
                    // Rename .new to .snap (approve the snapshot)
                    rename(&new_path, &settled_path).expect("Failed to approve snapshot");
                    println!("Snapshot '{}' approved.", name);
                    break;
                }
                "r" | "reject" => {
                    // Remove the .new file (reject the snapshot)
                    remove_file(&new_path).expect("Failed to reject snapshot");
                    println!("Snapshot '{}' rejected.", name);
                    break;
                }
                "s" | "skip" => {
                    // Leave the .new file as is (skip the snapshot)
                    println!("Snapshot '{}' skipped.", name);
                    break;
                }
                _ => {
                    println!("Please enter 'a' (approve), 'r' (reject), or 's' (skip).");
                }
            }
        }
    }
}

/// Updated function as per your request
fn print_differences(old: &str, new: &str) {
    use std::cmp::max;

    let old_lines: Vec<&str> = old.lines().collect();
    let new_lines: Vec<&str> = new.lines().collect();

    let max_lines = max(old_lines.len(), new_lines.len());

    // Count the number of differing lines
    let mut diff_count = 0;

    for i in 0..max_lines {
        let old_line = old_lines.get(i).unwrap_or(&"");
        let new_line = new_lines.get(i).unwrap_or(&"");

        if old_line != new_line {
            diff_count += 1;
        }
    }

    if diff_count > 40 {
        // Print entire old snapshot in max red
        println!("\x1b[31m{}\x1b[0m", old);

        // Print entire new snapshot in max green
        println!("\x1b[32m{}\x1b[0m", new);
    } else {
        for i in 0..max_lines {
            let old_line = old_lines.get(i).unwrap_or(&"");
            let new_line = new_lines.get(i).unwrap_or(&"");

            if old_line != new_line {
                // Print old line in red
                println!("\x1b[31m- {}\x1b[0m", old_line);
                // Print new line in green
                println!("\x1b[32m+ {}\x1b[0m", new_line);
            }
        }
    }
}
