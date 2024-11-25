import os
from setuptools import setup, Extension
from setuptools.command.build_ext import build_ext

class BuildExtCommand(build_ext):
    def initialize_options(self):
        build_ext.initialize_options(self)
        self.cpp = False

    def finalize_options(self):
        build_ext.finalize_options(self)
        if os.environ.get('USE_CPP'):
            self.cpp = True

    def run(self):
        if not self.cpp:
            self.extensions = []
        build_ext.run(self)

cpp_extension = Extension('_geohash',
    sources=['src/geohash.cpp'],
    define_macros=[('PYTHON_MODULE', 1)])

with open('README.md', 'r', encoding='utf-8') as f:
    long_description = f.read()

setup(
    name='python-geohash',
    version='0.8.5',
    description='Fast, accurate python geohashing library with optional C++ support',
    long_description=long_description,
    long_description_content_type='text/markdown',
    author='Hiroaki Kawai',
    author_email='kawai@iij.ad.jp',
    maintainer='Steven Lee',
    maintainer_email='alitrack.com@gmail.com',
    url='https://github.com/alitrack/python-geohash2',
    project_urls={
        'Bug Tracker': 'https://github.com/alitrack/python-geohash2/issues',
        'Source Code': 'https://github.com/alitrack/python-geohash2',
        'Documentation': 'https://github.com/alitrack/python-geohash2',
    },
    license='MIT',
    classifiers=[
        'Development Status :: 5 - Production/Stable',
        'Intended Audience :: Developers',
        'License :: OSI Approved :: MIT License',
        'Programming Language :: Python :: 3',
        'Programming Language :: Python :: 3.6',
        'Programming Language :: Python :: 3.7',
        'Programming Language :: Python :: 3.8',
        'Programming Language :: Python :: 3.9',
        'Programming Language :: Python :: 3.10',
        'Programming Language :: Python :: 3.11',
        'Topic :: Scientific/Engineering :: GIS',
        'Topic :: Software Development :: Libraries :: Python Modules',
    ],
    keywords='geohash gis geographic coordinates latitude longitude',
    python_requires='>=3.6',
    py_modules=['geohash', 'quadtree', 'jpgrid', 'jpiarea'],
    ext_modules=[cpp_extension],
    cmdclass={
        'build_ext': BuildExtCommand,
    },
    extras_require={
        'dev': [
            'pytest>=6.0',
            'pytest-cov>=2.0',
            'flake8>=3.9.0',
            'black>=21.0',
            'isort>=5.0',
        ],
    },
)
