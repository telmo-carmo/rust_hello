use serde::{Deserialize, Serialize};

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

fn do_post(ui: u32) -> Result<(), ureq::Error> {
    // Create the data payload
    let new_post = Req1 {
        title: "Hello from Rust".to_string(),
        body: "Posting data using the ureq crate.".to_string(),
        user_id: ui,
    };

    // Send the POST request
    // ureq automatically sets the Content-Type to application/json when using send_json
    let mut response =
        ureq::post("https://jsonplaceholder.typicode.com/posts").send_json(&new_post)?;

    let result: Resp1 = response.body_mut().read_json::<Resp1>()?;

    // 3. Print the result
    println!("Post created successfully!");
    println!("{:#?}", result);

    Ok(())
}

fn main() -> Result<(), ureq::Error> {
    for i in 0..5 {
        do_post(i)?;
    }
    Ok(())
}
