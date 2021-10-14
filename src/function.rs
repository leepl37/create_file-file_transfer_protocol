use ftp::{FtpStream};
use std::sync::mpsc;
use std::thread;
use dotenv::dotenv;
use ftp::types::FileType;
use crate::model::{FtpInsert, FtpInsertData};
use std::str::from_utf8;

pub fn scan_port(host: &str, port: u16) -> Result<FtpStream, bool> {
    let host = host.to_string();
    let port = port;
    let (sender, receiver) = mpsc::channel();
    let t = thread::spawn(move || {
        // match sender.send(net::TcpStream::connect((host.as_str(), port))) {
        match sender.send(FtpStream::connect((host.as_str(), port))) {
            Ok(()) => {
                println!("접속 되었습니다.");
            }, // everything good
            Err(_) => {
                println!("접속이 불가능 합니다.");
            }, // we have been released, don't panic
        }
    });

    thread::sleep(std::time::Duration::new(5, 0));

    println!("쓰레드 타임아웃");
    match receiver.try_recv() {
        Ok(Ok(handle)) =>{
            println!("FtpStream Return");
            Ok(handle) // we have a connection
        }
        Ok(Err(_)) => Err(false), // connecting failed
        Err(mpsc::TryRecvError::Empty) => {
            println!("ftp 접속 에러");
            drop(receiver);
            drop(t);
            // connecting took more than 5 seconds
            Err(false)
        },
        Err(mpsc::TryRecvError::Disconnected) => unreachable!(),
    }
}


pub fn ftp_data_insert(ftpinfo:FtpInsert) -> Result<bool, Box<dyn std::error::Error>> {
    dotenv().ok();

    let test_value = std::env::var("FTP_TEST")?;
    let mut request:bool;
    if test_value.eq("true") {
        request = true;
    }else {
        request = false;
    }
    let mut info_for_ftp = get_info_for_ftp(request, ftpinfo)?;


    if let Ok(mut ftp_stream) = scan_port(info_for_ftp.ftp_server.as_str(), 21) {
        println!("ftp 스트림 :{:?}", ftp_stream);
        let _ = ftp_stream.login(info_for_ftp.ftp_id.as_str(), info_for_ftp.ftp_pw.as_str())?;
        println!("경로: {}", &info_for_ftp.ftp_path);
        let mut flap_data = format!("{}\t{}\n", info_for_ftp.ftp_flap_number, info_for_ftp.ftp_action);
        println!("flap_data: {}", &flap_data);
        ftp_stream.cwd(info_for_ftp.ftp_path.as_str());
        println!("경로 지정: cwd");
        ftp_stream.transfer_type(FileType::Binary);
        println!("바이너리형태로 변경");
        let pwd = ftp_stream.pwd()?;
        println!("pwd:{}", pwd);
        ftp_stream.put(info_for_ftp.ftp_file_name.as_str(), &mut flap_data.as_bytes());

       /*
       ftp_stream 을 통해 만들어진 파일을 다시 retr 하여 파일에 적힌 text와 실제 입력한 값이 일치하는지 확인하는 코드.
        실제 서버에선 사용하지 않을 예정..
        */
        // let ftp_text_file_name = info_for_ftp.ftp_file_name.clone();
        // let remote_file = ftp_stream.simple_retr(ftp_text_file_name.as_str())?; //에러처리하기
        // let vec = remote_file.into_inner();
        // let result = from_utf8(&*vec);
        // println!("result: {:?}", result);
        // let t = format!("{}\t{}\n", info_for_ftp.ftp_flap_number, info_for_ftp.ftp_action);
        // if result.unwrap().eq(&t){
        //     println!("일치");
        // }else{
        //     println!("불일치");
        // }

        // if from_utf8(&remote_file.into_inner()).unwrap().eq(&flap_data) {
        // let x: i32 = ftpinfo.action.parse().unwrap();
        // if x == 6 {
        //     ftp_stream.quit();
        //     // }
            println!("파일이 생성되었습니다.");
        Ok(true)
        }else{
        Ok(false)
    }
}

fn get_info_for_ftp(test:bool, data:FtpInsert) -> Result<FtpInsertData, Box<dyn std::error::Error>>{
    dotenv().ok();
    if test {
        let info = FtpInsertData {
            ftp_id: std::env::var("FTP_SERVER_ID")?,
            ftp_pw: std::env::var("FTP_SERVER_PW")?,
            ftp_server: std::env::var("FTP_SERVER")?,
            ftp_file_name: std::env::var("FTP_FILE_NAME")?,
            ftp_path: std::env::var("FTP_FILE_PATH")?,
            ftp_flap_number: data.flap_number,
            ftp_action: data.action,
            port_which: std::env::var("FTP_PORT")?
        };
        Ok(info)
    }else{
        let info = FtpInsertData {
            ftp_id: data.id,
            ftp_pw: data.pw,
            ftp_server: data.ip,
            ftp_file_name: std::env::var("FTP_FILE_NAME")?,
            ftp_path: std::env::var("FTP_FILE_PATH")?,
            ftp_flap_number: data.flap_number,
            ftp_action: data.action,
            port_which: std::env::var("FTP_PORT")?
        };
        Ok(info)
    }
}