use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

/// A simple Spin HTTP component.
#[http_component]
fn kindness_checker(req: Request) -> Result<Response> {
    println!("{:?}", req);
    let (parts, body) = req.into_parts();
    //convert body to a vec
    //let body_vec = body::to_bytes(body).await.unwrap().to_vec();
  
    let out = format!("{{\"method\" : \"{}\",\n\
                            \"uri\" : \"{}\",\n\
                            \"headers\" : {:?},\n\
                            \"version\" : \"{:?}\",\n\
                            \"body\" : \"{:?}\",}}",
                    parts.method,
                    parts.uri,
                    parts.headers,
                    parts.version,
                    String::from_utf8(body.unwrap().to_vec()).unwrap());
    
    Ok(http::Response::builder()
        .status(200)
        .header("foo", "bar")
        .body(Some(out.into()))?)
}
