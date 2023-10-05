use lambda_http::{ run, service_fn, Body, Error, Request, Response };
use tokio_postgres::Client;
use native_tls::{ Certificate, TlsConnector };
use postgres_native_tls::MakeTlsConnector;
use std::fs;
use once_cell::sync::OnceCell;

static DB: OnceCell<Client> = OnceCell::new();

async fn get_users() -> Result<Response<Body>, Error> {
    let db = DB.get().ok_or("Internal Server Error")?;
    match db.query("SELECT * FROM users;", &[]).await {
        Ok(result) => {
            for row in result {
                let username: &str = row.get("username");
                print!("{}", username);
                return Ok(Response::new(username.into()));
            }
            Err("No Users".into())
        }
        Err(err) => {
            print!("{}", err);
            Err("Failed to get users".into())
        }
    }

    // Ok(
    //     Response::builder()
    //         .status(200)
    //         .header("content-type", "text/html")
    //         .body("Resp".into())
    //         .map_err(Box::new)?
    // )
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request

    let method = event.method();
    if method == "GET" {
        get_users().await
    } else if method == "POST" {
        // post_users(event).await
        Ok(Response::new("WIP".into()))
    } else {
        let resp = Response::builder()
            .status(200)
            .header("content-type", "text/html")
            .body("Unsupported".into())
            .map_err(Box::new)?;
        Ok(resp)
    }

    // let who = event
    //     .query_string_parameters_ref()
    //     .and_then(|params| params.first("name"))
    //     .unwrap_or("world");
    // let message = format!("Hello {who}, this is an AWS Lambda HTTP request");

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber
        ::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let cert = fs::read("src/cert.pem")?;
    let cert = Certificate::from_pem(&cert)?;

    let connector = TlsConnector::builder().add_root_certificate(cert).build()?;
    let connector = MakeTlsConnector::new(connector);
    let (client, connection) = tokio_postgres::connect(
        "host=sauce.cz7i0u2dk84i.us-west-1.rds.amazonaws.com user=postgres password=<INSERTPASSWORDHERE>",
        connector
    ).await?;
    let _ = DB.set(client);
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    run(service_fn(function_handler)).await
}
