use ic_cdk::api::management_canister::http_request::http_request;
use ic_cdk::api::management_canister::http_request::CanisterHttpRequestArgument;
use ic_cdk::api::management_canister::http_request::HttpHeader;
use ic_cdk::api::management_canister::http_request::HttpMethod;
use ic_cdk::println;



#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
#[derive(Debug, serde::Deserialize)]
pub struct Response {
    translation_text: String,
}

#[ic_cdk::update]
async fn translate(text:String) -> Result<String,String> {
   
    let arg = CanisterHttpRequestArgument {
        url: "https://api-inference.huggingface.co/models/google-t5/t5-base".to_string(),
        max_response_bytes: None,
        method: HttpMethod::POST,
        headers: vec![HttpHeader {
            name: "Authorization".to_string(),
            value: format!("Bearer {}", token),
        }],
        body: Some(format!(r#"{{"inputs": "{}"}}"#, text).into()),
        transform: None,
    };

    let res = http_request(
        arg,
        (1_603_112_888 +text.len()*500).try_into().unwrap(),
    )
    .await.map_err(|error| format!("Error {:?},{:?}",error.0,error.1)).unwrap();
    println!("response below:");
    
    println!("{:?}", res);
    println!("response body below:");
    println!("{:?}", String::from_utf8(res.0.body.clone()));
    let formatted_res : (Response,) = serde_json::from_slice(&res.0.body).map_err(|e| format!("Failed to parse response: {}", e))?;

        
    Ok(formatted_res.0.translation_text)
}




