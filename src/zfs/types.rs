#[allow(dead_code)]
pub struct Dataset {
    pub name: String,
    pub mounted: bool,
    pub available: f64,
    pub used: f64,
    pub compression: String,
    pub mount_point: String
}

impl Dataset {
    /// Contructor to make Dataset object from output of `zfs list` command
    pub fn from_line(line: &str) -> Dataset {
        let dp_raw: Vec<&str> = line.split('\t').collect();
        return Dataset {
            name: dp_raw[0].to_string(),
            mounted: dp_raw[1] == "yes",
            available: Dataset::parse_size(dp_raw[2]),
            used: Dataset::parse_size(dp_raw[3]),
            compression: dp_raw[4].to_string(),
            mount_point: dp_raw[5].to_string()
        };
    }

    /// Contructor to make vector of Datasets from `zfs list` command
    pub fn from_stdout(stdout: String) -> Vec<Dataset> {
        let mut vec = Vec::new();
        for line in  stdout.trim().lines() {
            vec.push(Dataset::from_line(line));
        }
        return vec;
    }
    fn parse_size(size: &str) -> f64 {
        let mut size_string = size.to_string();
        size_string.pop();
        let parsed: Result<f64, _> = size_string.parse();
        return parsed.unwrap();
    }
}

/// All kinds of ZFS errors
#[derive(Debug)]
pub enum ZfsError {
    /// Returned when `zfs list` command has non 0 exit code
    CmdFailed(String),
	NotImplemented
}

/// Zfs module generic result type.
pub type ZfsResult<T> = Result<T, ZfsError>;
