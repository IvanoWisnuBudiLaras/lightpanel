#[derive(Debug)]
pub struct AppUnit {
    pub service_name: String,
    pub description: String,
    pub working_directory: String,
    pub environment_file: String,
    pub exec_start: String,
}
