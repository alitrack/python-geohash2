# python-geohash

python-geohash is a fast, accurate python geohashing library.

## Features

- Pure Python implementation by default
- Optional C++ extension for better performance
- Supports Python 3.6+
- Includes quadtree, Japanese grid (jpgrid), and Japanese area (jpiarea) modules

## Installation

### Basic Installation (Pure Python)

```bash
pip install .
```

### With C++ Extension

```bash
USE_CPP=1 pip install .
```

## Usage

```python
import geohash

# Encode coordinates to geohash
latitude = 35.658581
longitude = 139.745433
hash_str = geohash.encode(latitude, longitude)  # -> 'xn76urx6k'

# Decode geohash to coordinates
lat, lon = geohash.decode(hash_str)  # -> (35.658581, 139.745433)

# Get bounding box
bbox = geohash.bbox(hash_str)
# Returns: {'s': south, 'w': west, 'n': north, 'e': east}

# Get neighbors
neighbors = geohash.neighbors(hash_str)
# Returns: {'n': north, 'ne': northeast, 'e': east, 'se': southeast,
#           's': south, 'sw': southwest, 'w': west, 'nw': northwest}
```

## History

- python-geohash2: Pure Python implementation by default, optional C++ extension
- python-geohash 0.8: Introduced uint64 representation
- python-geohash 0.7.1: Started supporting python3k
- python-geohash 0.3: Added C extension support

## License

Code is licensed under Apache License 2.0, MIT Licence and NEW BSD License.
You can choose one of these licences.

### Apache License 2.0

Copyright 2011 Hiroaki Kawai

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

### MIT License

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.

### New BSD License

Copyright (c) 2011, Hiroaki Kawai
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:
    * Redistributions of source code must retain the above copyright
      notice, this list of conditions and the following disclaimer.
    * Redistributions in binary form must reproduce the above copyright
      notice, this list of conditions and the following disclaimer in the
      documentation and/or other materials provided with the distribution.
    * Neither the name of the python-geohash nor the
      names of its contributors may be used to endorse or promote products
      derived from this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL Hiroaki Kawai BE LIABLE FOR ANY
DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
(INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

## Credits

Original author: Hiroaki Kawai <kawai@iij.ad.jp>  
Current maintainer: Steven Lee <alitrack.com@gmail.com>

## Links

- [Source Code](https://github.com/alitrack/python-geohash2)
- [Issue Tracker](https://github.com/alitrack/python-geohash2/issues)
- [Documentation](https://github.com/alitrack/python-geohash2)
