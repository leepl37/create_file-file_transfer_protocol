use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug)]
struct FtpInfo {
    pub flap_number:String,
    pub action:String

}
#[derive(Serialize, Deserialize, Debug)]
pub struct FTPFail {
    pub r#type:String,
    pub msg:String,
    pub result:bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FtpInsert {
    pub ip:String,
    pub id:String,
    pub pw:String,
    pub flap_number:String,
    pub action:String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct JsonReturn {
    pub r#type: Option<String>,
    pub msg:String,
    pub result: bool,
}

impl JsonReturn {
    pub fn return_result(name: Option<String>, err:String) -> Self {
        match name {
            None => JsonReturn {
                r#type: None,
                msg:err,
                result: false,
            },
            Some(u) => {
                let mut name = String::from("");
                if u.contains("admin") {
                    name = "admin".to_string();
                } else if u.contains("ftp") {
                    name = "ftp".to_string();
                } else if u.contains("signup") {
                    name = "signup_user".to_string();
                } else {
                    name = "user".to_string();
                }
                JsonReturn {
                    r#type: Some(name),
                    msg: err,
                    result: true,
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct FtpInsertData {
    pub ftp_id:String,
    pub ftp_pw:String,
    pub ftp_server:String,
    pub ftp_file_name:String,
    pub ftp_path:String,
    pub ftp_flap_number:String,
    pub ftp_action:String,
    pub port_which:String
}