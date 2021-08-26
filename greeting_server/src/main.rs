use greeting_client::{GreetingService, GetGreetingRequest, GetGreetingResponse, Greeting, CreateGreetingRequest, CreateGreetingResponse};
use warp_rpc::generate_service_server;
use warp_rpc::error::ServiceError;
use warp_rpc::error::Status::{BadRequest, NotImplemented};

#[tokio::main]
async fn main() {
    env_logger::init();
    server::start(GreetingServiceServer {}, 8000).await;
}

struct GreetingServiceServer {}

#[async_trait::async_trait]
impl GreetingService for GreetingServiceServer {
    async fn get_greeting(&self, req: GetGreetingRequest) -> Result<GetGreetingResponse, ServiceError> {
        log::info!("handling {:?}", req);
        let greeting = match req.language.as_str() {
            "en" => format!("Good morning {}", req.person_name),
            "fr" => format!("Bonjour {}", req.person_name),
            "jp" => format!("おはよう {}", req.person_name),
            _ => return Err(ServiceError {
                status: BadRequest,
                message: Some("Unsuppported language".to_string()),
                cause: None,
            })
        };
        let resp = GetGreetingResponse {
            greeting: Greeting { greeting, language: "en".to_string() }
        };
        Ok(resp)
    }

    async fn create_greeting(&self, req: CreateGreetingRequest) -> Result<CreateGreetingResponse, ServiceError> {
        log::info!("handling {:?}", req);
        Err(ServiceError { status: NotImplemented, message: None, cause: None })
    }
}

generate_service_server!(
    greeting { get_greeting }
);
