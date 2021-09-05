use flate2::read::GzDecoder;
use once_cell::sync::OnceCell;
use rand::Rng;
use std::fs::File;
use std::path::PathBuf;
use std::{env, fs, path::Path};
use tar::Archive;
use uuid::Uuid;

static RUNTIME_PATH: OnceCell<PathBuf> = OnceCell::new();

pub fn set_runtime_path(path: &Path) {
    RUNTIME_PATH.set(path.to_owned()).unwrap();
}

pub fn get_runtime_path() -> &'static PathBuf {
    RUNTIME_PATH.get().expect("Runtime path is not set")
}

pub fn initialize_test(project_path: &Path) -> Result<(), std::io::Error> {
    prepare_test_workspace(project_path)
}

pub fn cleanup_test(project_path: &Path) -> Result<(), std::io::Error> {
    delete_test_workspace(project_path)
}

pub fn get_project_path() -> PathBuf {
    let current_dir_path_result = env::current_dir();
    match current_dir_path_result {
        Ok(path_buf) => path_buf,
        Err(e) => panic!("directory is not found, {}", e),
    }
}

// This will generate the UUID needed when creating the container.
pub fn generate_uuid() -> Uuid {
    let mut rng = rand::thread_rng();
    const CHARSET: &[u8] = b"0123456789abcdefABCDEF";

    let rand_string: String = (0..32)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    match Uuid::parse_str(&rand_string) {
        Ok(uuid) => uuid,
        Err(e) => panic!("can not parse uuid, {}", e),
    }
}

// Temporary files to be used for testing are created in the `integration-workspace`.
fn prepare_test_workspace(project_path: &Path) -> Result<(), std::io::Error> {
    let integration_test_workspace_path = project_path.join("integration-workspace");
    let create_dir_result = fs::create_dir_all(&integration_test_workspace_path);
    if fs::create_dir_all(&integration_test_workspace_path).is_err() {
        return create_dir_result;
    }
    let tar_file_name = "bundle.tar.gz";
    let tar_path = integration_test_workspace_path.join(tar_file_name);
    fs::copy(
        tar_file_name,
        &integration_test_workspace_path.join(tar_file_name),
    )?;
    let tar_gz = File::open(tar_path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(integration_test_workspace_path)?;

    Ok(())
}

// This deletes all temporary files.
fn delete_test_workspace(project_path: &Path) -> Result<(), std::io::Error> {
    fs::remove_dir_all(project_path.join("integration-workspace"))?;

    Ok(())
}
