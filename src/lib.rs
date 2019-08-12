use std::result::Result;
use std::process::{Command, Stdio};

#[derive(Debug, Clone, Copy)]
pub enum FuseStatus {
    Mountting,
    Unmountting,
    Unmountted,
}

#[derive(Debug, Clone)]
pub struct FuseMount {
    mount_point: String,
}

impl FuseMount {
    pub fn mount(proc: &str, bucket_name: &str, mount_point: &str, ak_sk: &str, url: &str) -> Result<Self, String> {
        let child = Command::new(proc.to_string())
            .arg(bucket_name.to_string())
            .arg(mount_point.to_string())
            .arg("-o")
            .arg(ak_sk.to_string())
            .arg("-o")
            .arg(url.to_string())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute child");

        let ecode = child.wait_with_output()
            .expect("failed to wait on child");
                
        // command error
        if ecode.status.code().unwrap() == 1 {
            return Err("Parameters error!".to_string());
        }

        Ok(FuseMount{
            mount_point: mount_point.to_string(),
        })
    }

    pub fn unmount(&mut self) -> Result<(), String> {

        let child = Command::new("fusermount")
            .arg("-u")
            .arg(&self.mount_point.to_string())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute child");

        let ecode = child.wait_with_output()
                    .expect("failed to wait on child");

        if ecode.status.code().unwrap() != 0 {
            return Err("unmount error, the virtul disk maybe using!".to_string())
        }

        Ok(())
    }
}
