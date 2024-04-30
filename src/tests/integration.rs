use tokio::sync::oneshot;
use tonic::transport::Server;
use protos::auth::auth_client::AuthClient;
use protos::auth::auth_server::AuthServer;
use protos::auth::{LoginRequest, RegisterRequest};
use crate::tests::{TestContext};
use futures_util::FutureExt;
use validator::ValidateLength;
use crate::models::user::{NewUser, User};
use crate::tests::fixtures::{load_fixtures_from_yaml};

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
        email: "bad_email".to_string()
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

#[tokio::test]
async fn login() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let ctx = TestContext::new("postgres://postgres:postgres@127.0.0.1", "login", "redis://:@localhost:6379", 6064);

    let fixtures = load_fixtures_from_yaml::<User>("./src/tests/fixtures.yaml")
        .expect("Failed to load user fixtures");

    ctx.mock_database(|mut conn| {
        for user in &fixtures {
            let new_user = NewUser {
                email: user.email.as_str(),
                otp_secret: user.otp_secret.as_str()
            };
            User::create(&mut conn, new_user)
                .expect("Failed to create user");
        }
    });

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

    let request = tonic::Request::new(LoginRequest {
        email: fixtures[0].email.clone(),
    });

    let response = client.login(request).await?;
    let response = response.into_inner();

    if response.code.length().unwrap() != 6 {
        panic!("Expected 6 digit code");
    }

    tx.send(()).unwrap();
    jh.await.unwrap();
    Ok(())
}

#[tokio::test]
async fn login_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let ctx = TestContext::new("postgres://postgres:postgres@127.0.0.1", "login_not_exist", "redis://:@localhost:6379", 6065);

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

    let request = tonic::Request::new(LoginRequest {
        email: "none@none.fr".to_string(),
    });

    match client.login(request).await {
        Ok(_) => panic!("Expected error"),
        Err(e) => {
            assert_eq!(e.code(), tonic::Code::NotFound);
        }
    }

    tx.send(()).unwrap();
    jh.await.unwrap();
    Ok(())
}

#[tokio::test]
async fn login_bad_argument() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let ctx = TestContext::new("postgres://postgres:postgres@127.0.0.1", "login_bad_argument", "redis://:@localhost:6379", 6066);

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

    let request = tonic::Request::new(LoginRequest {
        email: "bad_email".to_string(),
    });

    match client.login(request).await {
        Ok(_) => panic!("Expected error"),
        Err(e) => {
            assert_eq!(e.code(), tonic::Code::InvalidArgument);
        }
    }

    tx.send(()).unwrap();
    jh.await.unwrap();
    Ok(())
}
