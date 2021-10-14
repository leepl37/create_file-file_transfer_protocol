mod model;
mod function;
use log::{error, info, warn, debug, trace};
use log4rs;
use actix_web::{App, HttpServer, Responder, HttpResponse, get, web};

use crate::model::{FTPFail, JsonReturn, FtpInsert};
pub use crate::function::{ftp_data_insert};
use dotenv::dotenv;

#[get("/")]
async fn test() -> impl Responder{
    HttpResponse::Ok().body("hello world")
}


#[get("/api/ftpupload")]
async fn insert_ftp(info: web::Form<FtpInsert>) -> impl Responder {

    let mut insert:bool = false;
    let info_data = info.into_inner();
    let mut msg = String::from("");

    match ftp_data_insert(info_data) {
        Ok(insert_return) => {
            println!("ftp 주소 체크 {}", insert_return);
            if !insert_return {
                let err_str = format!("ftp 파일을 업로드 중 에러 발생. 에러코드 : IP 접속 에러");
                //ftp 파일 만드는 데 실패 시,
                let fail = FTPFail {
                    r#type: "ftp".to_string(),
                    msg: err_str,
                    result: false
                };
                msg = fail.msg.clone();
                return HttpResponse::Ok()
                    .header("Access-Control-Allow-Origin", "*")
                    .header("Access-Control-Allow-Credentials", "true")
                    .json(fail)
            }
            insert = insert_return;
            let json_return: JsonReturn;
            if insert {
                json_return = JsonReturn::return_result(Some("ftp".to_string()), "전송완료".to_string());
            } else {
                json_return = JsonReturn::return_result(None, msg);
            }
            HttpResponse::Ok()
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Credentials", "true")
                .json(json_return)

        }
        Err(err) => {
            let err_str = format!("ftp 파일을 업로드 중 에러 발생. 에러코드 :{}", err);

            //ftp dyn error,
            let fail = FTPFail {
                r#type: "ftp".to_string(),
                msg: err_str,
                result: false
            };
            return HttpResponse::Ok()
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Credentials", "true")
                .json(fail)
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    HttpServer::new(move || {
        App::new()
            .service(insert_ftp)
            .service(test)
    })
        // .bind("0.0.0.0:8383")?
        .bind("0.0.0.0:8081")?
        .run()
        .await
}