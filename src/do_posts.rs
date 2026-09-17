use serde::{Deserialize, Serialize};
use std::time::Instant; 
use std::thread;
use std::env;

// Define the structure of the data you want to send
#[derive(Serialize, Deserialize, Debug)]
struct Req1 {
    title: String,
    body: String,
    #[serde(rename = "userId")]
    user_id: u32,
}

// Define the response structure (JSONPlaceholder returns the created item with an ID)
#[derive(Deserialize, Debug)]
struct Resp1 {
    id: u32,
    title: String,
    body: String,
    #[serde(rename = "userId")]
    user_id: u32,
}

struct ReqInfo {
    id : u32,
    status: u32,
    usecs: u64,
}

fn do_post(ui: u32, vf : bool) -> Result<ReqInfo, ureq::Error> {
    // Create the data payload
    let dts = chrono::Local::now()
        .format("%Y-%m-%d %H:%M:%S.%6f")
        .to_string();
    let new_post = Req1 {
        title: dts,
        body: "Posting data using the ureq crate.".to_string(),
        user_id: ui,
    };

    let total_start = Instant::now();

    // Send the POST request
    // ureq automatically sets the Content-Type to application/json when using send_json
    let mut response =
        ureq::post("https://jsonplaceholder.typicode.com/posts").send_json(&new_post)?;

    let result: Resp1 = response.body_mut().read_json::<Resp1>()?;

    let total_elapsed_micros = total_start.elapsed().as_micros();

    let r_status = response.status().as_u16() as u32;
    let r_usecs = total_elapsed_micros as u64;
    if r_status == 201 && vf {
        // Print the result
        println!(
            "Post created [ id: {}, title: {}, body: {}, user_id: {} ]",
            result.id, result.title, result.body, result.user_id
        );
    }

    Ok(ReqInfo{id: ui, status: r_status, usecs: r_usecs})
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut vf = false;
    let mut nt: u32 = 5;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-v" => {
                vf = true;
            }
            "-n" => {
                i += 1;

                let value = args
                    .get(i)
                    .ok_or("-n requires an integer")?;

                nt = value
                        .parse::<u32>()
                        .map_err(|_| "-n requires an integer")?;
                
            }
            unknown => {
                println!("unknown argument: {unknown}");
            }
        }
        i += 1;
    }

    let rv: Vec<ReqInfo> = thread::scope(|scope| {
        let handles: Vec<_> = (0..nt)
            .map(|i| scope.spawn(move || do_post(i,vf)))
            .collect();

        handles
            .into_iter()
            .map(|handle| handle.join().expect("request thread panicked"))
            .collect::<Result<Vec<_>, _>>()
    })?;

    let mut avg_ms = 0.0f32;
    for ri in &rv  {
        print!("Req ID: {}, Status: {}, Time taken (usecs): {}\n", ri.id, ri.status, ri.usecs);
        avg_ms = ri.usecs as f32 / 1000.0;
    }
    println!("Average time req taken: {} ms", avg_ms/(rv.len() as f32));
    Ok(())
}
