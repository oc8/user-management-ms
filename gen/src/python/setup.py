from setuptools import setup, find_packages

setup(
    name='grpc-user-management-auth',
    version='0.1.0-pre',
    packages=find_packages(),
    package_dir={'': 'auth'},
    install_requires=[
        'grpcio',
    ],
)