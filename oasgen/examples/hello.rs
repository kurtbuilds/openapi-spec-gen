#![allow(unused)]
#![allow(non_camel_case_types)]

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use openapiv3::{MediaType, Operation, ReferenceOr, RequestBody, Response, Responses, Schema, StatusCode};
use oasgen::{OaSchema, Server};
// use oasgen_core::OaOperation;
use serde::{Serialize, Deserialize};
use actix_web::web::Json;
use oasgen_core::{FunctionMetadata, OaOperation, TypedResponseFuture};
use tokio;



#[derive(OaSchema)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub phone: String,
    pub user_status: Option<i32>,
}



#[derive(OaSchema, Debug, Serialize)]
pub struct SendCodeResponse {
    pub found_account: bool,
}


pub async fn send_code(mobile: String) -> SendCodeResponse {
    SendCodeResponse { found_account: false }
}

// would be autogenerated
pub struct __send_code__metadata;

impl FunctionMetadata for __send_code__metadata {
    fn operation_id() -> Option<String> {
        Some("send_code_customized_shizzle".to_string())
    }
    fn summary() -> Option<String> {
        Some("custom custom read all about it".to_string())
    }
    fn description() -> Option<String> {
        Some("zzzzzzzzzzzSend a verification code to a mobile phone number".to_string())
    }
}

#[derive(OaSchema, Deserialize)]
pub struct SendCode {
    pub mobile: String,
}


pub fn send_code_transformed(body: Json<SendCode>) -> TypedResponseFuture<impl Future<Output=Json<SendCodeResponse>>, __send_code__metadata> {
    TypedResponseFuture::new(async move {
        // body of original function...
        Json(SendCodeResponse { found_account: false })
    })
}

fn get_operation<Op, Signature>(operation: Op) -> Operation
    where
        Op: OaOperation<Signature>,
{
    Op::operation()
}


// This is generic. You need 1 for each number of function arguments, but it doesn't need to be for
// every function, which is a good thing!!!

#[tokio::main]
async fn main() {
    let r = send_code_transformed(Json(SendCode{mobile: "hi".to_string()})).await;
    // let op = get_operation(send_code_transformed);
    let s = Server::new()
        .get("/auth/send-code", send_code_transformed);
    serde_yaml::to_writer(&std::fs::File::create("examples/hello.yaml").unwrap(), &s.openapi).unwrap();
    println!("{:#?}", s.openapi);
    // let s = User::schema();
    // println!("{:#?}", s);
}