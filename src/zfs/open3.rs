use zfs::types::{Dataset, ZfsError, ZfsResult};

use std::process::{Command};
const LIST_ARGS: [&'static str; 4] = ["-H", "-r", "-oname,mounted,available,used,compressratio,mountpoint", "-tfilesystem"];

pub fn list_datasets() -> ZfsResult<Vec<Dataset>> {
	let cmd = Command::new("zfs")
	.arg("list")
	.args(&LIST_ARGS)
	.output()
	.unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

	let result: ZfsResult<Vec<Dataset>>;
	if cmd.status.success() {
		let stdout = String::from_utf8(cmd.stdout).unwrap();
		result =  Ok(Dataset::from_stdout(stdout));
	} else {
		let stderr = String::from_utf8(cmd.stderr).unwrap();
		result = Err(ZfsError::CmdFailed(stderr));
	}
	return result;
}