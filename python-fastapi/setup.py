import os

from setuptools import setup, find_packages

setup(name='contact-server',
      version='0.1',
      description="serveur de contact starlet/fastapi",
      use_pipfile=True,
      classifiers=[
          "Programming Language :: Python",
          "Framework :: Starlet",
          "Framework :: FastAPI",
          "Topic :: Internet :: WWW/HTTP",
          "Topic :: Internet :: WWW/HTTP :: WSGI :: Application",
      ],
      url='http://github.com/',
      setup_requires=['setuptools-pipfile'],
      include_package_data=True,
      zip_safe=False,
      test_suite="nose.collector",
      )

