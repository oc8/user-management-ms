# Changelog

## [1.2.0](https://github.com/oc8/user-management-ms/compare/v1.1.2...v1.2.0) (2024-08-08)


### Features

* add build proto script for imports ([e786f4c](https://github.com/oc8/user-management-ms/commit/e786f4ca3a039c02bc9928206ee923acb7e9a89d))
* add openapi codegen ([df9bd1a](https://github.com/oc8/user-management-ms/commit/df9bd1a24c366f26884882c39ab7db76d823b619))
* add sqlx queries cache ([fe8f137](https://github.com/oc8/user-management-ms/commit/fe8f13724492630a576d9144a3927e4b8afcf634))


### Bug Fixes

* remove sqlx auto migrate ([3926722](https://github.com/oc8/user-management-ms/commit/39267227c8cac21170bab5f48a9727527361b5bc))

## [1.1.2](https://github.com/oc8/user-management-ms/compare/v1.1.1...v1.1.2) (2024-06-25)


### Bug Fixes

* allow dead code ([4c8741d](https://github.com/oc8/user-management-ms/commit/4c8741d580a78c47bad1e011905afe1deb025ab8))
* cargo allow dead code for pr-tests ([85b9a3a](https://github.com/oc8/user-management-ms/commit/85b9a3a9ab5bf08bcca33c205c2444d908c58388))
* create user if not exist ([814f4dd](https://github.com/oc8/user-management-ms/commit/814f4dd6c51c16766f67118bbd76a22e8aceb643))
* migrate dockerfile to sqlx, allow dead code & fix pr-tests workflow ([b0c3978](https://github.com/oc8/user-management-ms/commit/b0c3978bbcdee29fa78f7c97660b8941e3852b87))
* pull_request_target =&gt; pull_request ([a72683c](https://github.com/oc8/user-management-ms/commit/a72683c53e1685f83d2a7e8dbd57fad527c3917e))

## [1.1.1](https://github.com/oc8/user-management-ms/compare/v1.1.0...v1.1.1) (2024-06-05)


### Bug Fixes

* remove build files ([0374828](https://github.com/oc8/user-management-ms/commit/03748280b91619fd8ce92f2c700baebcad14a79e))


### Miscellaneous Chores

* release 1.1.1 ([3514a16](https://github.com/oc8/user-management-ms/commit/3514a16f503844598720ae9d43c0a7d4b831a0fa))

## [1.1.0](https://github.com/oc8/user-management-ms/compare/1.0.3...v1.1.0) (2024-06-05)


### Features

* add doc generator, renamed login method to generate_otp ([2bdbe6a](https://github.com/oc8/user-management-ms/commit/2bdbe6afec3d3f5c174eb33bbb6c44a82799ddaa))
* add error logging ([21b22d9](https://github.com/oc8/user-management-ms/commit/21b22d9f930acd76b41ff3d3bdac6c7f5dcfda6a))
* add GenerateMagicLink, ValidateMagicLink, ValidateOTP, ValidateToken, RefreshToken & Logout tests ([8f9937b](https://github.com/oc8/user-management-ms/commit/8f9937b074d35e940d4ba6fc4d46e57d6af58f0b))
* add login tests, fixtures & mock_database function ([fea8a5e](https://github.com/oc8/user-management-ms/commit/fea8a5ed8ad76b04b231840d598db58bda67e4ef))
* add Makefile ([d06c497](https://github.com/oc8/user-management-ms/commit/d06c4979f2c47b60a9a722e72f243807ce19f593))
* add server reflection ([b647b5e](https://github.com/oc8/user-management-ms/commit/b647b5ed53b49747e4cb763e243fa5550db4c16f))
* add testing flow ([f4cf55b](https://github.com/oc8/user-management-ms/commit/f4cf55bfbc32ac67ea7978dcbb24c480f5c326bb))


### Bug Fixes

* remove panic ([785fcf6](https://github.com/oc8/user-management-ms/commit/785fcf630a8053f055325e3a28d87330456407b5))
* tests fix replaced login method by generate_otp ([d296ad4](https://github.com/oc8/user-management-ms/commit/d296ad47fd31bb11ba97ae832988712b30d36752))
* validate email in validate_otp_request ([aec0d16](https://github.com/oc8/user-management-ms/commit/aec0d16070b6cef4fed70a66fa0ac1c923cf8a66))
