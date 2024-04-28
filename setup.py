from setuptools import setup, find_packages

setup(
    name='auth',
    version='1.0.2',
    package_dir={'': 'gen/src/python'},
    install_requires=[
        'betterproto',
    ],
)