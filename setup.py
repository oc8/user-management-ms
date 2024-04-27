from setuptools import setup, find_packages

setup(
    name='auth',
    version='1.0.2',
    package_dir={'auth': 'gen/src/python'},
    install_requires=[
        'betterproto',
    ],
)