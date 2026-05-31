from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
	rust_extensions=[RustExtension('_geohash', 'Cargo.toml', binding=Binding.PyO3)],
	zip_safe=False,
)
