use greeting_client::{GreetingService, GetGreetingRequest, GetGreetingResponse};
use greeting_client::service::GreetingClient;
use warp_rpc::error::ServiceError;

#[tokio::main]
async fn main() {
    env_logger::init();
    let client = GreetingClient::new("http://0.0.0.0:8000");
    let requests = vec![
        GetGreetingRequest { person_name: "Mike".to_string(), language: "en".to_string() },
        GetGreetingRequest { person_name: "Mike".to_string(), language: "fr".to_string() },
        GetGreetingRequest { person_name: "Mike".to_string(), language: "jp".to_string() },
        GetGreetingRequest { person_name: "Mike".to_string(), language: "ru".to_string() },
    ];

    for request in requests {
        let result = client.get_greeting(request).await;
        log_result(result)
    }
}

fn log_result(res: Result<GetGreetingResponse, ServiceError>) {
    match res {
        Ok(resp) => log::info!("{:?}", resp),
        Err(e) => log::error!("{}", e),
    }
}
