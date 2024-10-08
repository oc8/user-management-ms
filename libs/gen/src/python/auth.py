# Generated by the protocol buffer compiler.  DO NOT EDIT!
# sources: auth/auth.proto
# plugin: python-betterproto
from dataclasses import dataclass

import betterproto
import grpclib


@dataclass
class User(betterproto.Message):
    """* Represent a user"""

    id: str = betterproto.string_field(1)
    email: str = betterproto.string_field(2)


@dataclass
class Tokens(betterproto.Message):
    """* Represent a pair of tokens"""

    access_token: str = betterproto.string_field(1)
    refresh_token: str = betterproto.string_field(2)
    expires_in: int = betterproto.uint64_field(3)


@dataclass
class RegisterRequest(betterproto.Message):
    """* Used to register a new user"""

    email: str = betterproto.string_field(1)


@dataclass
class RegisterResponse(betterproto.Message):
    pass


@dataclass
class GenerateOTPRequest(betterproto.Message):
    """* Used to generate an OTP code"""

    email: str = betterproto.string_field(1)
    pkce_challenge: str = betterproto.string_field(2)


@dataclass
class GenerateOTPResponse(betterproto.Message):
    """* Response to an OTP generation request"""

    pass


@dataclass
class GenerateMagicLinkRequest(betterproto.Message):
    """* Used to generate a magic link"""

    email: str = betterproto.string_field(1)
    pkce_challenge: str = betterproto.string_field(2)


@dataclass
class GenerateMagicLinkResponse(betterproto.Message):
    """* Response to a magic link generation request"""

    pass


@dataclass
class ValidateMagicLinkRequest(betterproto.Message):
    """* Used to validate a magic link"""

    email: str = betterproto.string_field(1)
    code: str = betterproto.string_field(2)
    pkce_verifier: str = betterproto.string_field(3)


@dataclass
class ValidateMagicLinkResponse(betterproto.Message):
    """* Response to a magic link validation request"""

    tokens: "Tokens" = betterproto.message_field(1)


@dataclass
class ValidateOTPRequest(betterproto.Message):
    """* Used to validate an OTP"""

    email: str = betterproto.string_field(1)
    otp: str = betterproto.string_field(2)
    pkce_verifier: str = betterproto.string_field(3)


@dataclass
class ValidateOTPResponse(betterproto.Message):
    """* Response to an OTP validation request"""

    tokens: "Tokens" = betterproto.message_field(1)


@dataclass
class ValidateTokenRequest(betterproto.Message):
    """* Used to validate a token"""

    access_token: str = betterproto.string_field(1)


@dataclass
class ValidateTokenResponse(betterproto.Message):
    """* Response to a token validation request"""

    valid: bool = betterproto.bool_field(1)


@dataclass
class RefreshTokenRequest(betterproto.Message):
    """* Used to refresh a token"""

    refresh_token: str = betterproto.string_field(1)


@dataclass
class RefreshTokenResponse(betterproto.Message):
    """* Response to a token refresh request, return a new pair of tokens"""

    tokens: "Tokens" = betterproto.message_field(1)


@dataclass
class LogoutRequest(betterproto.Message):
    """* Used to logout a user, invalidate the refresh token"""

    refresh_token: str = betterproto.string_field(1)


@dataclass
class LogoutResponse(betterproto.Message):
    pass


class AuthStub(betterproto.ServiceStub):
    """* Auth service"""

    async def register(self, *, email: str = "") -> RegisterResponse:
        """/ Register a new user"""

        request = RegisterRequest()
        request.email = email

        return await self._unary_unary(
            "/auth.Auth/Register",
            request,
            RegisterResponse,
        )

    async def generate_o_t_p(
        self, *, email: str = "", pkce_challenge: str = ""
    ) -> GenerateOTPResponse:
        """/ Generate an OTP code"""

        request = GenerateOTPRequest()
        request.email = email
        request.pkce_challenge = pkce_challenge

        return await self._unary_unary(
            "/auth.Auth/GenerateOTP",
            request,
            GenerateOTPResponse,
        )

    async def generate_magic_link(
        self, *, email: str = "", pkce_challenge: str = ""
    ) -> GenerateMagicLinkResponse:
        """/ Generate a magic link"""

        request = GenerateMagicLinkRequest()
        request.email = email
        request.pkce_challenge = pkce_challenge

        return await self._unary_unary(
            "/auth.Auth/GenerateMagicLink",
            request,
            GenerateMagicLinkResponse,
        )

    async def validate_magic_link(
        self, *, email: str = "", code: str = "", pkce_verifier: str = ""
    ) -> ValidateMagicLinkResponse:
        """/ Validate a magic link"""

        request = ValidateMagicLinkRequest()
        request.email = email
        request.code = code
        request.pkce_verifier = pkce_verifier

        return await self._unary_unary(
            "/auth.Auth/ValidateMagicLink",
            request,
            ValidateMagicLinkResponse,
        )

    async def validate_o_t_p(
        self, *, email: str = "", otp: str = "", pkce_verifier: str = ""
    ) -> ValidateOTPResponse:
        """/ Validate an OTP"""

        request = ValidateOTPRequest()
        request.email = email
        request.otp = otp
        request.pkce_verifier = pkce_verifier

        return await self._unary_unary(
            "/auth.Auth/ValidateOTP",
            request,
            ValidateOTPResponse,
        )

    async def validate_token(self, *, access_token: str = "") -> ValidateTokenResponse:
        """/ Validate a token"""

        request = ValidateTokenRequest()
        request.access_token = access_token

        return await self._unary_unary(
            "/auth.Auth/ValidateToken",
            request,
            ValidateTokenResponse,
        )

    async def refresh_token(self, *, refresh_token: str = "") -> RefreshTokenResponse:
        """/ Refresh a token"""

        request = RefreshTokenRequest()
        request.refresh_token = refresh_token

        return await self._unary_unary(
            "/auth.Auth/RefreshToken",
            request,
            RefreshTokenResponse,
        )

    async def logout(self, *, refresh_token: str = "") -> LogoutResponse:
        """/ Logout a user"""

        request = LogoutRequest()
        request.refresh_token = refresh_token

        return await self._unary_unary(
            "/auth.Auth/Logout",
            request,
            LogoutResponse,
        )
