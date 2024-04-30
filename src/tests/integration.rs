use tokio::sync::oneshot;
use tonic::transport::Server;
use protos::auth::auth_client::AuthClient;
use protos::auth::auth_server::AuthServer;
use protos::auth::{GenerateMagicLinkRequest, LoginRequest, LogoutRequest, RefreshTokenRequest, RegisterRequest, ValidateMagicLinkRequest, ValidateOtpRequest, ValidateTokenRequest};
use crate::tests::{TestContext};
use futures_util::FutureExt;
use validator::ValidateLength;
use crate::models::user::{NewUser, User};
use crate::tests::fixtures::{load_fixtures_from_yaml};

#[tokio::test]
async fn register() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = TestContext::new("postgres://postgres:postgres@127.0.0.1", "register", "redis://:@localhost:6380", 6061);
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
    let ctx = TestContext::new("postgres://postgres:postgres@127.0.0.1", "register_already_exist", "redis://:@localhost:6380", 6062);
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
    let ctx = TestContext::new("postgres://postgres:postgres@127.0.0.1", "register_bad_argument", "redis://:@localhost:6380", 6063);
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
    let ctx = TestContext::new("postgres://postgres:postgres@127.0.0.1", "login", "redis://:@localhost:6380", 6064);

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

    assert_eq!(response.code.length().unwrap(), 6);

    tx.send(()).unwrap();
    jh.await.unwrap();
    Ok(())
}

#[tokio::test]
async fn login_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let ctx = TestContext::new("postgres://postgres:postgres@127.0.0.1", "login_not_exist", "redis://:@localhost:6380", 6065);

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
    let ctx = TestContext::new("postgres://postgres:postgres@127.0.0.1", "login_bad_argument", "redis://:@localhost:6380", 6066);

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

#[tokio::test]
async fn generate_magic_link() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let ctx = TestContext::new("postgres://postgres:postgres@127.0.0.1", "generate_magic_link", "redis://:@localhost:6380", 6067);

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

    let request = tonic::Request::new(GenerateMagicLinkRequest {
        email: fixtures[0].email.clone(),
    });

    let response = client.generate_magic_link(request).await?;
    let response = response.into_inner();

    assert_eq!(response.code.is_empty(), false);

    tx.send(()).unwrap();
    jh.await.unwrap();
    Ok(())
}

#[tokio::test]
async fn generate_magic_link_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let ctx = TestContext::new("postgres://postgres:postgres@127.0.0.1", "generate_magic_link_not_exist", "redis://:@localhost:6380", 6068);

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

    let request = tonic::Request::new(GenerateMagicLinkRequest {
        email: "none@none.fr".to_string(),
    });

    let response = client.generate_magic_link(request).await?;
    let response = response.into_inner();

    assert_eq!(response.code.is_empty(), false);

    tx.send(()).unwrap();
    jh.await.unwrap();
    Ok(())
}

#[tokio::test]
async fn generate_magic_link_bad_argument() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let ctx = TestContext::new("postgres://postgres:postgres@127.0.0.1", "generate_magic_link_bad_argument", "redis://:@localhost:6380", 6069);

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

    let request = tonic::Request::new(GenerateMagicLinkRequest {
        email: "bad_email".to_string(),
    });

    match client.generate_magic_link(request).await {
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
async fn validate_magic_link() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let ctx = TestContext::new("postgres://postgres:postgres@127.0.0.1", "validate_magic_link", "redis://:@localhost:6380", 6070);

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

    let request = tonic::Request::new(GenerateMagicLinkRequest {
        email: "test@test.fr".to_string(),
    });

    let response = client.generate_magic_link(request).await?;
    let response = response.into_inner();

    let request = tonic::Request::new(ValidateMagicLinkRequest {
        email: "test@test.fr".to_string(),
        code: response.code,
    });

    let response = client.validate_magic_link(request).await?;
    let response = response.into_inner();

    assert_eq!(response.tokens.is_none(), false);

    tx.send(()).unwrap();
    jh.await.unwrap();
    Ok(())
}

#[tokio::test]
async fn validate_magic_link_email_not_found() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let ctx = TestContext::new("postgres://postgres:postgres@127.0.0.1", "validate_magic_link_email_not_found", "redis://:@localhost:6380", 6071);

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

    let request = tonic::Request::new(GenerateMagicLinkRequest {
        email: "test@test.fr".to_string(),
    });

    let response = client.generate_magic_link(request).await?;
    let response = response.into_inner();

    let request = tonic::Request::new(ValidateMagicLinkRequest {
        email: "none@none.fr".to_string(),
        code: response.code,
    });

    match client.validate_magic_link(request).await {
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
async fn validate_magic_link_code_not_found() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let ctx = TestContext::new("postgres://postgres:postgres@127.0.0.1", "validate_magic_link_code_not_found", "redis://:@localhost:6380", 6072);

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

    let request = tonic::Request::new(GenerateMagicLinkRequest {
        email: "test@test.fr".to_string(),
    });

    client.generate_magic_link(request).await?;

    let request = tonic::Request::new(ValidateMagicLinkRequest {
        email: "test@test.fr".to_string(),
        code: "bad_code".to_string()
    });

    match client.validate_magic_link(request).await {
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
async fn validate_magic_link_bad_email() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let ctx = TestContext::new("postgres://postgres:postgres@127.0.0.1", "validate_magic_link_bad_email", "redis://:@localhost:6380", 6073);

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

    let request = tonic::Request::new(ValidateMagicLinkRequest {
        email: "bad_email".to_string(),
        code: "0000".to_string()
    });

    match client.validate_magic_link(request).await {
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
async fn validate_otp() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let ctx = TestContext::new("postgres://postgres:postgres@127.0.0.1", "validate_otp", "redis://:@localhost:6380", 6074);

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

    let request = tonic::Request::new(ValidateOtpRequest {
        email: fixtures[0].email.clone(),
        otp: response.code,
    });

    let response = client.validate_otp(request).await?;
    let response = response.into_inner();

    assert_eq!(response.tokens.is_none(), false);

    tx.send(()).unwrap();
    jh.await.unwrap();
    Ok(())
}

#[tokio::test]
async fn validate_otp_bad_email() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let ctx = TestContext::new("postgres://postgres:postgres@127.0.0.1", "validate_otp_bad_email", "redis://:@localhost:6380", 6075);

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

    let request = tonic::Request::new(ValidateOtpRequest {
        email: "bad_email".to_string(),
        otp: "000000".to_string()
    });

    match client.validate_otp(request).await {
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
async fn validate_otp_bad_code() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let ctx = TestContext::new("postgres://postgres:postgres@127.0.0.1", "validate_otp_bad_code", "redis://:@localhost:6380", 6076);

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

    let request = tonic::Request::new(ValidateOtpRequest {
        email: "none@none.fr".to_string(),
        otp: "".to_string()
    });

    match client.validate_otp(request).await {
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
async fn validate_otp_email_not_found() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let ctx = TestContext::new("postgres://postgres:postgres@127.0.0.1", "validate_otp_email_not_found", "redis://:@localhost:6380", 6077);

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

    let request = tonic::Request::new(ValidateOtpRequest {
        email: "none@none.fr".to_string(),
        otp: "000000".to_string()
    });

    match client.validate_otp(request).await {
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
async fn validate_otp_code_not_found() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let ctx = TestContext::new("postgres://postgres:postgres@127.0.0.1", "validate_otp_code_not_found", "redis://:@localhost:6380", 6078);

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

    let request = tonic::Request::new(ValidateOtpRequest {
        email: fixtures[0].email.clone(),
        otp: "000000".to_string()
    });

    match client.validate_otp(request).await {
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
async fn validate_token_with_otp() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let ctx = TestContext::new("postgres://postgres:postgres@127.0.0.1", "validate_token_with_otp", "redis://:@localhost:6380", 6079);

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

    let request = tonic::Request::new(ValidateOtpRequest {
        email: fixtures[0].email.clone(),
        otp: response.code,
    });

    let response = client.validate_otp(request).await?;
    let response = response.into_inner();

    assert_eq!(response.tokens.is_none(), false);

    let request = tonic::Request::new(ValidateTokenRequest {
        access_token: response.tokens.unwrap().access_token,
    });

    let response = client.validate_token(request).await?;
    let response = response.into_inner();

    assert_eq!(response.valid, true);

    tx.send(()).unwrap();
    jh.await.unwrap();
    Ok(())
}

#[tokio::test]
async fn validate_token_with_magic_link() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let ctx = TestContext::new("postgres://postgres:postgres@127.0.0.1", "validate_token_with_magic_link", "redis://:@localhost:6380", 6080);

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

    let request = tonic::Request::new(GenerateMagicLinkRequest {
        email: fixtures[0].email.clone(),
    });

    let response = client.generate_magic_link(request).await?;
    let response = response.into_inner();

    let request = tonic::Request::new(ValidateMagicLinkRequest {
        email: fixtures[0].email.clone(),
        code: response.code,
    });

    let response = client.validate_magic_link(request).await?;
    let response = response.into_inner();

    assert_eq!(response.tokens.is_none(), false);

    let request = tonic::Request::new(ValidateTokenRequest {
        access_token: response.tokens.unwrap().access_token,
    });

    let response = client.validate_token(request).await?;
    let response = response.into_inner();

    assert_eq!(response.valid, true);

    tx.send(()).unwrap();
    jh.await.unwrap();
    Ok(())
}

#[tokio::test]
async fn validate_token_bad_argument() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let ctx = TestContext::new("postgres://postgres:postgres@127.0.0.1", "validate_token_bad_argument", "redis://:@localhost:6380", 6081);

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

    let request = tonic::Request::new(ValidateTokenRequest {
        access_token: "".to_string(),
    });

    match client.validate_token(request).await {
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
async fn validate_token_bad_token() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let ctx = TestContext::new("postgres://postgres:postgres@127.0.0.1", "validate_token_bad_token", "redis://:@localhost:6380", 6082);

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

    let request = tonic::Request::new(ValidateTokenRequest {
        access_token: "bad_token".to_string(),
    });

    match client.validate_token(request).await {
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
async fn refresh_token() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let ctx = TestContext::new("postgres://postgres:postgres@127.0.0.1", "refresh_token", "redis://:@localhost:6380", 6083);

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

    let request = tonic::Request::new(GenerateMagicLinkRequest {
        email: "test@test.com".to_string(),
    });

    let response = client.generate_magic_link(request).await?;
    let response = response.into_inner();

    let request = tonic::Request::new(ValidateMagicLinkRequest {
        email: "test@test.com".to_string(),
        code: response.code,
    });

    let response = client.validate_magic_link(request).await?;
    let response = response.into_inner();

    let request = tonic::Request::new(RefreshTokenRequest {
        refresh_token: response.tokens.unwrap().refresh_token,
    });

    let response = client.refresh_token(request).await?;
    let response = response.into_inner();

    assert_eq!(response.tokens.is_none(), false);

    tx.send(()).unwrap();
    jh.await.unwrap();
    Ok(())
}

#[tokio::test]
async fn refresh_token_bad_argument() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let ctx = TestContext::new("postgres://postgres:postgres@127.0.0.1", "refresh_token_bad_argument", "redis://:@localhost:6380", 6084);

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

    let request = tonic::Request::new(RefreshTokenRequest {
        refresh_token: "".to_string(),
    });

    match client.refresh_token(request).await {
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
async fn refresh_token_invalid_token() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let ctx = TestContext::new("postgres://postgres:postgres@127.0.0.1", "refresh_token_invalid_token", "redis://:@localhost:6380", 6085);

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

    let request = tonic::Request::new(RefreshTokenRequest {
        refresh_token: "bad_token".to_string(),
    });

    match client.refresh_token(request).await {
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
async fn logout() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let ctx = TestContext::new("postgres://postgres:postgres@127.0.0.1", "logout", "redis://:@localhost:6380", 6086);

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

    let request = tonic::Request::new(GenerateMagicLinkRequest {
        email: "test@test.com".to_string(),
    });

    let response = client.generate_magic_link(request).await?;
    let response = response.into_inner();

    let request = tonic::Request::new(ValidateMagicLinkRequest {
        email: "test@test.com".to_string(),
        code: response.code,
    });

    let response = client.validate_magic_link(request).await?;
    let response = response.into_inner();


    let request = tonic::Request::new(LogoutRequest {
        refresh_token: response.tokens.unwrap().refresh_token,
    });

    client.logout(request).await?;

    tx.send(()).unwrap();
    jh.await.unwrap();
    Ok(())
}

#[tokio::test]
async fn logout_bad_argument() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let ctx = TestContext::new("postgres://postgres:postgres@127.0.0.1", "logout_bad_argument", "redis://:@localhost:6380", 6087);

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

    let request = tonic::Request::new(LogoutRequest {
        refresh_token: "".to_string()
    });

    match client.logout(request).await {
        Ok(_) => panic!("Expected error"),
        Err(e) => {
            assert_eq!(e.code(), tonic::Code::InvalidArgument);
        }
    }

    tx.send(()).unwrap();
    jh.await.unwrap();
    Ok(())
}
