from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
	# Source builds stay interpreter-specific; wheel builds request cp38-abi3 below.
	rust_extensions=[
		RustExtension('_geohash', 'Cargo.toml', binding=Binding.PyO3, py_limited_api="auto")
	],
	options={"bdist_wheel": {"py_limited_api": "cp38"}},
	zip_safe=False,
)
