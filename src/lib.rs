#[macro_use] extern crate pam;

use pam::module::{PamHandle, PamHooks};
use pam::constants::{PamResultCode, PamFlag, PAM_PROMPT_ECHO_ON};
use pam::conv::PamConv;

use std::str::FromStr;
use std::ffi::CStr;
use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;

macro_rules! pam_try {
    ($e:expr) => (
        match $e {
            Ok(v) => v,
            Err(e) => return e,
        }
    );
    ($e:expr, $err:expr) => (
        match $e {
            Ok(v) => v,
            Err(e) => {
                println!("Error: {}", e);
                return $err;
            }
        }
    );
}

struct PamIamAuth;
pam_hooks!(PamIamAuth);


fn log(msg: &[u8]) {
    let mut file = OpenOptions::new().create(true).append(true).open("/tmp/foo.txt").unwrap();
    file.write_all(msg).unwrap();
    file.write("\n".as_bytes()).unwrap();
    file.sync_all().unwrap();
}

impl PamHooks for PamIamAuth {
    fn sm_authenticate(pamh: &PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        log(format!("I'm here, args: {:?}", _args).as_bytes());
        log(format!("user: {:?}", pamh.get_user(None)).as_bytes());
        return PamResultCode::PAM_SUCCESS;
    }

    fn sm_setcred(_pamh: &PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        log(format!("I'm in set credentials, args: {:?}", _args).as_bytes());
        PamResultCode::PAM_SUCCESS
    }

    fn acct_mgmt(_pamh: &PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        log(format!("I'm in acct_mgmt!").as_bytes());
        PamResultCode::PAM_SUCCESS
    }

    fn sm_chauthtok(_pamh: &PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        log(format!("I'm in sm_chauthtok!").as_bytes());
        PamResultCode::PAM_SUCCESS
    }

    fn sm_open_session(_pamh: &PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        log(format!("I'm in sm_open_session!").as_bytes());
        PamResultCode::PAM_SUCCESS
    }

    fn sm_close_session(_pamh: &PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        log(format!("I'm in sm_close_session!").as_bytes());
        PamResultCode::PAM_SUCCESS
    }
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
