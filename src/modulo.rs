use std::time::{Duration, Instant};
use reqwest::StatusCode;
use colored::*;


pub async  fn foo()-> u32{
    println!("In foo");
    5
}

pub async fn get_status(url: &str)->Result<StatusCode ,Box<dyn std::error::Error>>{
    let start_time = Instant::now();
    let status_code = reqwest::get(url).await?.status();
    let duration=start_time.elapsed().as_millis();
    println!("Took {}ms to fetch url '{}'", duration.to_string().bright_magenta(), url.to_string().bright_blue());
    Ok(status_code)

}
pub async fn heartbeat(mut num : u32){
    loop{
        println!("beating..{}", num.to_string().bright_red());
        tokio::time::sleep(Duration::from_millis(100)).await;
        num += 1;
        if num> 15 {break;}
    }
}