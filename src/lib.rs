use std::fs::{create_dir_all, read_to_string, write};

/// Performs a snapshot test against `./snapshots/`.
/// Creates a snapshot if it does not exist.
pub fn snap(name: impl ToString, value: impl ToString) {
    let name = name.to_string();
    let value = value.to_string();
    let mut snapshots_dir = std::env::current_dir().expect("Failed to get current directory");
    snapshots_dir.push("snapshots");

    // Ensure the snapshots directory exists
    if !snapshots_dir.exists() {
        create_dir_all(&snapshots_dir).expect("Failed to create snapshots directory");
    }

    // Paths for the settled and new snapshots
    let mut settled_path = snapshots_dir.clone();
    settled_path.push(format!("{}.snap", name));
    let mut new_path = snapshots_dir.clone();
    new_path.push(format!("{}.new", name));

    // Check if the settled snapshot exists
    if !settled_path.exists() {
        // If it doesn't exist, create new and inform the user
        write(&new_path, &value).expect("Failed to write new snapshot");
        println!("\x1b[33mCreated new snapshot: {}\x1b[0m", settled_path.display());
    } else {
        // Compare the new snapshot with the settled one
        let settled_value =
            read_to_string(&settled_path).expect("Failed to read settled snapshot");
        if settled_value != value {
            println!("\x1b[33mSnapshot '{}' changed.\x1b[0m", name);
            write(&new_path, &value).expect("Failed to write new snapshot");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Approve
    #[test]
    fn test_snap0() {
        snap("snap0", "2+2=4");
    }

    // Try reject
    #[test]
    fn test_snap01() {
        snap("snap0", "2+2=5");
    }

    // Try accept
    #[test]
    fn test_snap02() {
        snap("snap0", "2+2+1=5");
    }
}
