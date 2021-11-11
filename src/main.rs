use rand::{Rng, thread_rng, distributions::Alphanumeric};
use std::{
    io::{stdout, Write},
    fs::File,
    env::args,
    thread::sleep,
    time::Duration,
    iter,
};
use reqwest::{get};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut f = File::create("gen_code.txt")?;
    let mut rng = thread_rng();
    let args: Vec<String> = args().collect();
    let to_run: i32 = args[1].parse().unwrap();
     sleep(Duration::from_millis(700));

    let mut stdout = stdout();
    for _ in 0..(to_run) {
        let mut chars: String = String::from("https://discord.gift/");
        let key: String = iter::repeat(())
                .map(|()| rng.sample(Alphanumeric))
                .map(char::from)
                .take(19)
                
                .collect();
            chars.push_str(key.as_str());
            let mut value: String = check(chars.as_ref()).await?;
            print!("\r{}", value);
            stdout.flush().unwrap();
            sleep(Duration::from_millis(20));
            value.push('\n');
            f.write(value.as_bytes())?;
    }
    println!();
    f.sync_all()?;
    Ok(())
}

async fn check(url : &str) -> Result<String, Box<dyn std::error::Error>> {
        let url_base = format!("https://discordapp.com/api/v9/entitlements/gift-codes/{}?with_application=false&with_subscription_plan=true", url);
        let resp = get(url_base)
        .await?
        .status();        
        if resp == 200 {
            return Ok(format!("Valid | {}", url))
        }  else {
            return Ok(format!("InValid | {}", url))
        }

}

