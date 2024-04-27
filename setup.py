from setuptools import setup, find_packages

setup(
    name='grpc-user-management-auth',
    version='1.0.2',
    package_dir={'grpc_user_management_auth': 'gen/src/python'},
    install_requires=[
        'betterproto',
    ],
)