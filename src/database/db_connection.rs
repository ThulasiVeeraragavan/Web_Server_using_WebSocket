use std::error::Error;
use std::time::Instant;
use tiberius::{Client, Config, AuthMethod};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;
use tokio_util::compat::Compat;
use serde_json::{json, Value};
use crate::models::req_models::{GetUserRequest, LoginRequest};

pub async fn db_connection() ->Result<Client<Compat<TcpStream>>,Box<dyn std::error::Error+ Send + Sync>>{
    let start_time = Instant::now();
    let duration = start_time.elapsed();
    println!("Db connection start time: {:?}", duration);
    let mut config = Config::new();
    config.host("192.168.10.214");
    config.port(1433);
    config.database("testdb");
    config.authentication(AuthMethod::sql_server("sa", "gitech123*gitech"));
    config.trust_cert();
    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true).expect("error 2");
    let client = Client::connect(config, tcp.compat_write()).await?;
    Ok(client)
}

pub async fn login(request: LoginRequest) -> Result<Value, Box<dyn Error+ Send + Sync>> {
    let start_time = Instant::now();
    let mut client = db_connection().await?;
    let duration = start_time.elapsed();
    println!("Db connection end time: {:?}", duration);
    let query = format!("EXEC Sample '{}','{}'", request.first_name, request.last_name);
    println!("Db Request: {}", query);
    let res = client.query(query,&[]).await?;
    let res_value=res.into_results().await?;
    println!("Db Response: {:?}", res_value);
    let fullname:&str = res_value[0][0].try_get("FullName")?.unwrap_or("");
    let out_json = json!({"fullname":fullname,});
    let duration_res = start_time.elapsed();
    println!("Db response time: {:?}", duration_res);
    return Ok(out_json);
}
pub async fn get_user(request: GetUserRequest) -> Result<Value, Box<dyn Error+ Send + Sync>> {
    let conn = db_connection().await?;
    let query = format!("EXEC Sample '{}'", request.user_id);
    let mut client = conn;
    let res = client.query(query,&[]).await?;
    let res_value=res.into_results().await?;
    let fullname:&str = res_value[0][0].try_get("data")?.unwrap_or("");
    let out_json = json!({"data":fullname,});
    return Ok(out_json);
}