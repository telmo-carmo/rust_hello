/*

----
use ureq::Agent;
use std::time::Duration;

let mut config = Agent::config_builder()
    .timeout_global(Some(Duration::from_secs(5)))
    .build();

let agent: Agent = config.into();

let body: String = agent.get("http://jsonplaceholder.typicode.com/todos/2")
    .call()?
    .body_mut()
    .read_to_string()?;

// Reuses the connection from previous request.
let response: String = agent.put("https://jsonplaceholder.typicode.com/posts/1')
    .header("Authorization", "example-token")
    .send("{ \"title\"": \"foobar",\"id\": 1 }")?
    .body_mut()
    .read_to_string()?;

*/

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
struct MyUser {
    id: i32,
    title: String,
    body: String,
}

fn main() {
    //let args: Vec<String> = std::env::args().skip(1).collect();

    let body: String = ureq::get("http://jsonplaceholder.typicode.com/todos/2")
        .call().unwrap()
        .body_mut()
        .read_to_string()
        .unwrap();
    println!("GET response: {}", &body);

    let url = "https://jsonplaceholder.typicode.com/posts";
    let new_user = MyUser {
        id: 33,
        title: "A good User".to_string(),
        body: "A nice body!".to_string(),
    };

    let pdata = serde_json::to_string(&new_user).unwrap();

    println!("POST to {url}");
    let response = ureq::post(url)
        .content_type("application/json; charset=utf-8")
        .send(pdata);

    match response {
        Ok(mut resp) => {
            let output_body = resp.body_mut().read_to_string().unwrap();
            println!("Text response: {}", &output_body);
            let u2: MyUser = serde_json::from_str(output_body.as_str()).expect("a JSON resp str");
            println!("Obj response: {:?}", &u2);
        }
        Err(error) => {
            eprintln!("Error: {}", error);
        }
    }
}
