use aws_lambda_events::encodings::Body;
use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use http::header::HeaderMap;
use lambda_runtime::{handler_fn, Context, Error};
use serde_json;
use tokio;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

fn multiple_uuids(count: i32) -> String {
    let uuids: Vec<_> = (0..count).into_iter().map(|_| Uuid::new_v4()).collect();
    serde_json::to_string(&uuids).expect("dang")
}

pub(crate) async fn my_handler(
    event: ApiGatewayProxyRequest,
    _ctx: Context,
) -> Result<ApiGatewayProxyResponse, Error> {
    let uuid_string: String;
    let has_count = event.query_string_parameters.contains_key("count");
    if has_count {
        let count: i32 = event.query_string_parameters["count"]
            .parse()
            .expect("aww shucks");
        uuid_string = multiple_uuids(count)
    } else {
        uuid_string = Uuid::new_v4().to_string()
    }
    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(format!("{}", uuid_string))),
        is_base64_encoded: Some(false),
    };
    Ok(resp)
}
