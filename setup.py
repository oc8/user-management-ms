from setuptools import setup, find_packages

setup(
    name='auth',
    version='v1.1.0',
    package_dir={'': 'gen/src/python'},
    install_requires=[
        'betterproto',
    ],
)