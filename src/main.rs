mod modulo;
use modulo::*;
use std::{sync::mpsc::Receiver, time::{Duration, Instant}};
use reqwest::*;
use tokio::*;
use colored::*; 

#[tokio::main]
async fn main()->Result<()> {
let separador = "---------------------".to_string().bright_red();

    println!("{}","Pruebas de asynchronous".to_string().bright_cyan());
    let u = foo().await;
//Ahora corremos  futures primero uno luego otro
    let start_time = Instant::now();
    let client = reqwest::Client::new();
let res = client.get("https://google.com").send().await?;
let status_1 = res.status();
println!("status 1:{}", status_1.to_string().bright_cyan());

let client2 = reqwest::Client::new();
let res2 = client.get("https://yahoo.com").send().await?;
let status_2 = res.status();
println!("status 2:{}", status_2.to_string().bright_cyan());

println!("Overall execute time: {}ms", start_time.elapsed().as_millis().to_string().bright_green());

println!("{}",separador);

//Ahora corremos multiples futures at de mismo tiempo , con macro -->tokio::join!()
println!("{}","Corremos multiples futures, tokio::join!() .".to_string().bright_cyan());
let start_time1 = Instant::now();
tokio::spawn(modulo::heartbeat(0));

let (status_1, status_2)= tokio::join!(
        modulo::get_status("https://google.com"),
        modulo::get_status("https://www.youtube.com/watch?v=zeBu4t-rUz8")
    );

println!("status 1: {}", status_1.unwrap());
println!("status 2:{}", status_2.unwrap());

println!("Overall execute time: {}ms", start_time1.elapsed().as_millis().to_string().bright_green());

println!("{}",separador);
//Ahora macro de tokio select!()
println!("{}","Corremos multiples futures, tokio::select!() .".to_string().bright_cyan());
println!("{}","solo va a ejecutar el brazo que primero responda el resto lo desecha.".to_string().bright_cyan());

let start_time2 = Instant::now();
   //solo va a ejecutar el brazo que primero responda el resto lo desecha
    tokio::select!(
        stat =modulo::get_status("https://google.com")=> println!("Status 1: {}", stat.unwrap()) ,
       stat = modulo::get_status("https://www.youtube.com/watch?v=zeBu4t-rUz8")=> println!("Status 2: {}", stat.unwrap()) ,
    );

println!("Overall execute time: {}ms", start_time2.elapsed().as_millis().to_string().bright_green());
println!("{}",separador);
let separador = "---------------------".to_string().bright_red();
    println!("{}","Pruebas de channels".to_string().bright_cyan());
    let (tx, mut rx)= tokio::sync::mpsc::channel(5);
    tokio::spawn(send_message(tx));
    while  let Some(message) =rx.recv().await  {
        println!("Receiver message: '{}'", message);
    }

Ok(())
}

async fn receive_message(mut rx: tokio::sync::mpsc::Receiver<String>){
    while let Some (message)= rx.recv().await{
        println!("Receiver message: '{}'", message);
    }
    
}

async fn send_message(tx: tokio::sync::mpsc::Sender<String>){
    let mut message_nr = 1;
    loop {
        tx.send(format!("Hola desde async [{message_nr}]"))
            .await
            .unwrap();
        message_nr += 1;
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;    
    }
}