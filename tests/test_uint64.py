import unittest
import math
import warnings
import geohash

class TestUint64(unittest.TestCase):
	def test_one(self):
		dataset = [
			(0x0000000000000000, -90.0, -180.0),
			(0x0800000000000000, -90.0, -135.0),
			(0x1000000000000000, -45.0, -180.0),
			(0x1800000000000000, -45.0, -135.0),
			(0x2000000000000000, -90.0, -90.0),
			(0x2800000000000000, -90.0, -45.0),
			(0x3000000000000000, -45.0, -90.0),
			(0x3800000000000000, -45.0, -45.0),
			(0x4000000000000000, 0.0, -180.0),
			(0x4800000000000000, 0.0, -135.0),
			(0x5000000000000000, 45.0, -180.0),
			(0x5800000000000000, 45.0, -135.0),
			(0x6000000000000000, 0.0, -90.0),
			(0x6800000000000000, 0.0, -45.0),
			(0x7000000000000000, 45.0, -90.0),
			(0x7800000000000000, 45.0, -45.0),
			(0x8000000000000000, -90.0, 0.0),
			(0x8800000000000000, -90.0, 45.0),
			(0x9000000000000000, -45.0, 0.0),
			(0x9800000000000000, -45.0, 45.0),
			(0xA000000000000000, -90.0, 90.0),
			(0xA800000000000000, -90.0, 135.0),
			(0xB000000000000000, -45.0, 90.0),
			(0xB800000000000000, -45.0, 135.0),
			(0xC000000000000000, 0.0, 0.0),
			(0xC800000000000000, 0.0, 45.0),
			(0xD000000000000000, 45.0, 0.0),
			(0xD800000000000000, 45.0, 45.0),
			(0xE000000000000000, 0.0, 90.0),
			(0xE800000000000000, 0.0, 135.0),
			(0xF000000000000000, 45.0, 90.0),
			(0xF800000000000000, 45.0, 135.0)
			]
		for data in dataset:
			self.assertEqual(data[0], geohash.encode_uint64(data[1], data[2]))
			latlon = geohash.decode_uint64(data[0])
			self.assertEqual(latlon[0], data[1])
			self.assertEqual(latlon[1], data[2])

	def test_north_pole_boundary_warns_and_uses_adjacent_cell(self):
		latitude = math.nextafter(90.0, -math.inf)
		expected = geohash.encode_uint64(latitude, 0.0)
		with warnings.catch_warnings(record=True) as captured:
			warnings.simplefilter("always")
			self.assertEqual(expected, geohash.encode_uint64(90.0, 0.0))
		self.assertEqual(1, len(captured))
		self.assertIn("nextafter(90.0, -inf)", str(captured[0].message))

if __name__=='__main__':
	unittest.main()
