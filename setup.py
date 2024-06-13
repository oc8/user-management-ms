from setuptools import setup, find_packages

setup(
    name='auth',
    version='v1.1.1',
    package_dir={'': 'libs/gen/src/python'},
    install_requires=[
        'betterproto',
    ],
)