use serde::{Deserialize, Serialize};
use warp_rpc::error::ServiceError;
use warp_rpc::generate_service_client;

#[async_trait::async_trait]
pub trait GreetingService {
    async fn get_greeting(&self, req: GetGreetingRequest) -> Result<GetGreetingResponse, ServiceError>;
    async fn create_greeting(&self, req: CreateGreetingRequest) -> Result<CreateGreetingResponse, ServiceError>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Greeting {
    pub greeting: String,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetGreetingRequest {
    pub person_name: String,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetGreetingResponse {
    pub greeting: Greeting,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGreetingRequest {}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGreetingResponse {}

generate_service_client!(
    greeting { get_greeting, create_greeting }
);
