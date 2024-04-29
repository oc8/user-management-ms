use tokio::sync::oneshot;
use tonic::transport::Server;
use protos::auth::auth_client::AuthClient;
use protos::auth::auth_server::AuthServer;
use protos::auth::{LoginRequest, RegisterRequest};
use crate::tests::{TestContext};
use futures_util::FutureExt;

#[tokio::test]
async fn register() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = TestContext::new("postgres://postgres:postgres@127.0.0.1", "register", "redis://:@localhost:6379", 6061);
    let (tx, rx) = oneshot::channel();
    let service = ctx.service.clone();

    let jh = tokio::spawn(async move {
        Server::builder()
            .add_service(AuthServer::new(service))
            .serve_with_shutdown(ctx.addr, rx.map(|_| ()))
            .await
            .unwrap();
    });

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let mut client = AuthClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(RegisterRequest {
        email: "test@test.com".to_string()
    });
    client.register(request).await?;

    tx.send(()).unwrap();
    jh.await.unwrap();
    Ok(())
}

#[tokio::test]
async fn register_already_exist() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = TestContext::new("postgres://postgres:postgres@127.0.0.1", "register_already_exist", "redis://:@localhost:6379", 6062);
    let (tx, rx) = oneshot::channel();
    let service = ctx.service.clone();

    let jh = tokio::spawn(async move {
        Server::builder()
            .add_service(AuthServer::new(service))
            .serve_with_shutdown(ctx.addr, rx.map(|_| ()))
            .await
            .unwrap();
    });

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let mut client = AuthClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(RegisterRequest {
        email: "test@test.com".to_string()
    });
    client.register(request).await?;

    let request = tonic::Request::new(RegisterRequest {
        email: "test@test.com".to_string()
    });

    match client.register(request).await {
        Ok(_) => panic!("Expected error"),
        Err(e) => {
            assert_eq!(e.code(), tonic::Code::AlreadyExists);
        }
    }

    tx.send(()).unwrap();
    jh.await.unwrap();
    Ok(())
}

#[tokio::test]
async fn register_bad_argument() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = TestContext::new("postgres://postgres:postgres@127.0.0.1", "register_bad_argument", "redis://:@localhost:6379", 6063);
    let (tx, rx) = oneshot::channel();
    let service = ctx.service.clone();

    let jh = tokio::spawn(async move {
        Server::builder()
            .add_service(AuthServer::new(service))
            .serve_with_shutdown(ctx.addr, rx.map(|_| ()))
            .await
            .unwrap();
    });

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let mut client = AuthClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(RegisterRequest {
        email: "test".to_string()
    });

    match client.register(request).await {
        Ok(_) => panic!("Expected error"),
        Err(e) => {
            assert_eq!(e.code(), tonic::Code::InvalidArgument);
        }
    }

    tx.send(()).unwrap();
    jh.await.unwrap();
    Ok(())
}
