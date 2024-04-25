from setuptools import setup, find_packages
import os

version = os.getenv('VERSION')

setup(
    name='grpc-user-management-auth',
    version=version,
    package_dir={'grpc_user_management_auth': 'gen/src/python/auth'},
    install_requires=[
        'grpcio',
    ],
)