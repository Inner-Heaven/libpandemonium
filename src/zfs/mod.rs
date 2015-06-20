use std::process::{Command, Output};


const LIST_ARGS: [&'static str; 4] = ["-H", "-r", "-oname,mounted,available,used,compressratio,mountpoint", "-tfilesystem"];
#[allow(dead_code)]
pub struct Dataset {
    name: String,
    mounted: bool,
    available: String,
    used: String,
    compression: String,
    mount_point: String
}

pub fn list_datasets() -> Result<Vec<Dataset>, String> {
    let cmd = Command::new("zfs")
        .arg("list")
        .args(&LIST_ARGS)
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

    let result: Result<Vec<Dataset>, String> = datasets_from_cmd(cmd);
    return result;
}
fn datasets_from_cmd(cmd: Output) -> Result<Vec<Dataset>, String> {
    let result: Result<Vec<Dataset>, String>;
    let status = cmd.status;
    if status.success() {
        let stdout = String::from_utf8(cmd.stdout).unwrap();
        let mut vec = Vec::new();
        for line in  stdout.trim().split('\n') {
            let dp_raw: Vec<&str> = line.split('\t').collect();
            let dataset = Dataset {
                name: dp_raw[0].to_string(),
                mounted: dp_raw[1] == "yes",
                available: dp_raw[2].to_string(),
                used: dp_raw[3].to_string(),
                compression: dp_raw[4].to_string(),
                mount_point: dp_raw[5].to_string()
            };
            vec.push(dataset);
        }
        result =  Ok(vec);
    } else {
        let stderr = String::from_utf8(cmd.stderr).unwrap();
        println!("Error listing zfs: {}", stderr);
        result = Err(stderr);
    }
    return result;
}
#[test]
fn it_should_list_datasets() {
    let datasets_result = list_datasets();
    assert_eq!(datasets_result.is_ok(), true);
    let datasets: Vec<Dataset> = datasets_result.unwrap();
    assert_eq!(datasets.len(), 4);
    assert_eq!(datasets[0].name, "tank".to_string());
}
