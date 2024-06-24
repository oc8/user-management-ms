use protos::auth::auth_client::AuthClient;
use protos::auth::{GenerateMagicLinkRequest, GenerateOtpRequest, LogoutRequest, RefreshTokenRequest, RegisterRequest, ValidateMagicLinkRequest, ValidateOtpRequest, ValidateTokenRequest};
use validator::ValidateLength;
use crate::{setup_test_context};

#[tokio::test]
async fn register() -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, tx, jh) = setup_test_context("register", 50200).await;
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
    let (ctx, tx, jh) = setup_test_context("register_already_exist", 50200).await;
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
    let (ctx, tx, jh) = setup_test_context("register_bad_argument", 50200).await;
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
    let (ctx, tx, jh) = setup_test_context("login", 50200).await;
    let mut client = AuthClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(RegisterRequest {
        email: "test@test.com".to_string()
    });
    client.register(request).await?;

    let request = tonic::Request::new(GenerateOtpRequest {
        email: "test@test.com".to_string()
    });

    let response = client.generate_otp(request).await?;
    let response = response.into_inner();

    assert_eq!(response.code.length().unwrap(), 6);

    tx.send(()).unwrap();
    jh.await.unwrap();
    Ok(())
}

#[tokio::test]
async fn login_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let (ctx, tx, jh) = setup_test_context("login_not_exist", 50200).await;
    let mut client = AuthClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(GenerateOtpRequest {
        email: "none@none.fr".to_string(),
    });

    match client.generate_otp(request).await {
        Ok(_) => panic!("Expected error"),
        Err(e) => {
            assert_eq!(e.code(), tonic::Code::NotFound);
        }
    }

    tx.send(()).unwrap();
    jh.await.unwrap();
    ctx.cleanup().await;
    Ok(())
}

#[tokio::test]
async fn login_bad_argument() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let (ctx, tx, jh) = setup_test_context("login_bad_argument", 50200).await;
    let mut client = AuthClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(GenerateOtpRequest {
        email: "bad_email".to_string(),
    });

    match client.generate_otp(request).await {
        Ok(_) => panic!("Expected error"),
        Err(e) => {
            assert_eq!(e.code(), tonic::Code::InvalidArgument);
        }
    }

    tx.send(()).unwrap();
    jh.await.unwrap();
    ctx.cleanup().await;
    Ok(())
}

#[tokio::test]
async fn generate_magic_link() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let (ctx, tx, jh) = setup_test_context("generate_magic_link", 50200).await;
    let mut client = AuthClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(RegisterRequest {
        email: "test@test.com".to_string()
    });
    client.register(request).await?;

    let request = tonic::Request::new(GenerateMagicLinkRequest {
        email: "test@test.com".to_string()
    });

    let response = client.generate_magic_link(request).await?;
    let response = response.into_inner();

    assert_eq!(response.code.is_empty(), false);

    tx.send(()).unwrap();
    jh.await.unwrap();
    ctx.cleanup().await;
    Ok(())
}

#[tokio::test]
async fn generate_magic_link_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let (ctx, tx, jh) = setup_test_context("generate_magic_link_not_exist", 50200).await;
    let mut client = AuthClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(GenerateMagicLinkRequest {
        email: "none@none.com".to_string(),
    });

    let response = client.generate_magic_link(request).await?;
    let response = response.into_inner();

    assert_eq!(response.code.is_empty(), false);

    tx.send(()).unwrap();
    jh.await.unwrap();
    ctx.cleanup().await;
    Ok(())
}

#[tokio::test]
async fn generate_magic_link_bad_argument() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let (ctx, tx, jh) = setup_test_context("generate_magic_link_bad_argument", 50200).await;
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
    let (ctx, tx, jh) = setup_test_context("validate_magic_link", 50200).await;
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

    assert_eq!(response.tokens.is_none(), false);

    tx.send(()).unwrap();
    jh.await.unwrap();
    Ok(())
}

#[tokio::test]
async fn validate_magic_link_email_not_found() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let (ctx, tx, jh) = setup_test_context("validate_magic_link_email_not_found", 50200).await;
    let mut client = AuthClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(GenerateMagicLinkRequest {
        email: "test@test.com".to_string(),
    });

    let response = client.generate_magic_link(request).await?;
    let response = response.into_inner();

    let request = tonic::Request::new(ValidateMagicLinkRequest {
        email: "none@none.com".to_string(),
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
    let (ctx, tx, jh) = setup_test_context("validate_magic_link_code_not_found", 50200).await;
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
    let (ctx, tx, jh) = setup_test_context("validate_magic_link_bad_email", 50200).await;
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
    let (ctx, tx, jh) = setup_test_context("validate_otp", 50200).await;
    let mut client = AuthClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(RegisterRequest {
        email: "test@test.com".to_string()
    });
    client.register(request).await?;

    let request = tonic::Request::new(GenerateOtpRequest {
        email: "test@test.com".to_string(),
    });

    let response = client.generate_otp(request).await?;
    let response = response.into_inner();

    let request = tonic::Request::new(ValidateOtpRequest {
        email: "test@test.com".to_string(),
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
    let (ctx, tx, jh) = setup_test_context("validate_otp_bad_email", 50200).await;
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
    let (ctx, tx, jh) = setup_test_context("validate_otp_bad_code", 50200).await;
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
    let (ctx, tx, jh) = setup_test_context("validate_otp_email_not_found", 50200).await;
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
    let (ctx, tx, jh) = setup_test_context("validate_otp_code_not_found", 50200).await;
    let mut client = AuthClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(RegisterRequest {
        email: "test@test.com".to_string()
    });
    client.register(request).await?;

    let request = tonic::Request::new(ValidateOtpRequest {
        email: "test@test.com".to_string(),
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
    let (ctx, tx, jh) = setup_test_context("validate_token_with_otp", 50200).await;
    let mut client = AuthClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(RegisterRequest {
        email: "test@test.com".to_string()
    });
    client.register(request).await?;

    let request = tonic::Request::new(GenerateOtpRequest {
        email: "test@test.com".to_string(),
    });

    let response = client.generate_otp(request).await?;
    let response = response.into_inner();

    let request = tonic::Request::new(ValidateOtpRequest {
        email: "test@test.com".to_string(),
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
    let (ctx, tx, jh) = setup_test_context("validate_token_with_magic_link", 50200).await;
    let mut client = AuthClient::connect(ctx.url.clone()).await.unwrap();

    let request = tonic::Request::new(RegisterRequest {
        email: "test@test.com".to_string()
    });
    client.register(request).await?;

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
    let (ctx, tx, jh) = setup_test_context("validate_token_bad_argument", 50200).await;
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
    let (ctx, tx, jh) = setup_test_context("validate_token_bad_token", 50200).await;
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
    let (ctx, tx, jh) = setup_test_context("refresh_token", 50200).await;
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
    let (ctx, tx, jh) = setup_test_context("refresh_token_bad_argument", 50200).await;
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
    let (ctx, tx, jh) = setup_test_context("refresh_token_invalid_token", 50200).await;
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
    let (ctx, tx, jh) = setup_test_context("logout", 50200).await;
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
    let (ctx, tx, jh) = setup_test_context("logout_bad_argument", 50200).await;
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
