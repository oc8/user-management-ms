# Generated by the protocol buffer compiler.  DO NOT EDIT!
# sources: auth/auth.proto
# plugin: python-betterproto
from dataclasses import dataclass

import betterproto
import grpclib


@dataclass
class User(betterproto.Message):
    id: str = betterproto.string_field(1)
    email: str = betterproto.string_field(2)


@dataclass
class Tokens(betterproto.Message):
    access_token: str = betterproto.string_field(1)
    refresh_token: str = betterproto.string_field(2)
    token_type: str = betterproto.string_field(3)
    expires_in: int = betterproto.uint64_field(4)
    user: "User" = betterproto.message_field(5)


@dataclass
class RegisterRequest(betterproto.Message):
    email: str = betterproto.string_field(1)


@dataclass
class RegisterResponse(betterproto.Message):
    pass


@dataclass
class LoginRequest(betterproto.Message):
    email: str = betterproto.string_field(1)


@dataclass
class LoginResponse(betterproto.Message):
    code: str = betterproto.string_field(1)


@dataclass
class GenerateMagicLinkRequest(betterproto.Message):
    email: str = betterproto.string_field(1)


@dataclass
class GenerateMagicLinkResponse(betterproto.Message):
    refresh_token: str = betterproto.string_field(1)
    expires_in: int = betterproto.uint64_field(2)


@dataclass
class ValidateOTPRequest(betterproto.Message):
    email: str = betterproto.string_field(1)
    otp: str = betterproto.string_field(2)


@dataclass
class ValidateOTPResponse(betterproto.Message):
    tokens: "Tokens" = betterproto.message_field(1)


@dataclass
class ValidateTokenRequest(betterproto.Message):
    access_token: str = betterproto.string_field(1)


@dataclass
class ValidateTokenResponse(betterproto.Message):
    valid: bool = betterproto.bool_field(1)


@dataclass
class RefreshTokenRequest(betterproto.Message):
    refresh_token: str = betterproto.string_field(1)


@dataclass
class RefreshTokenResponse(betterproto.Message):
    tokens: "Tokens" = betterproto.message_field(1)


@dataclass
class LogoutRequest(betterproto.Message):
    refresh_token: str = betterproto.string_field(1)


@dataclass
class LogoutResponse(betterproto.Message):
    pass


class AuthStub(betterproto.ServiceStub):
    async def register(self, *, email: str = "") -> RegisterResponse:
        request = RegisterRequest()
        request.email = email

        return await self._unary_unary(
            "/auth.Auth/Register",
            request,
            RegisterResponse,
        )

    async def login(self, *, email: str = "") -> LoginResponse:
        request = LoginRequest()
        request.email = email

        return await self._unary_unary(
            "/auth.Auth/Login",
            request,
            LoginResponse,
        )

    async def generate_magic_link(
        self, *, email: str = ""
    ) -> GenerateMagicLinkResponse:
        request = GenerateMagicLinkRequest()
        request.email = email

        return await self._unary_unary(
            "/auth.Auth/GenerateMagicLink",
            request,
            GenerateMagicLinkResponse,
        )

    async def validate_o_t_p(
        self, *, email: str = "", otp: str = ""
    ) -> ValidateOTPResponse:
        request = ValidateOTPRequest()
        request.email = email
        request.otp = otp

        return await self._unary_unary(
            "/auth.Auth/ValidateOTP",
            request,
            ValidateOTPResponse,
        )

    async def validate_token(self, *, access_token: str = "") -> ValidateTokenResponse:
        request = ValidateTokenRequest()
        request.access_token = access_token

        return await self._unary_unary(
            "/auth.Auth/ValidateToken",
            request,
            ValidateTokenResponse,
        )

    async def refresh_token(self, *, refresh_token: str = "") -> RefreshTokenResponse:
        request = RefreshTokenRequest()
        request.refresh_token = refresh_token

        return await self._unary_unary(
            "/auth.Auth/RefreshToken",
            request,
            RefreshTokenResponse,
        )

    async def logout(self, *, refresh_token: str = "") -> LogoutResponse:
        request = LogoutRequest()
        request.refresh_token = refresh_token

        return await self._unary_unary(
            "/auth.Auth/Logout",
            request,
            LogoutResponse,
        )