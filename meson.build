project('SynthLauncher', 'cpp', license: 'aGPL-3.0', version: '1.0.0', license_files: 'LICENSE', default_options: ['cpp_std=c++20', 'warning_level=3'])

cpp = meson.get_compiler('cpp')
ws2_32 = cpp.find_library('ws2_32')
crypt32 = cpp.find_library('crypt32')
openssl = dependency('openssl')

src = [
  'src/json/manifest.cc',
  'src/config/app.cc',
  'src/http/builder.cc',
  'src/http/parser.cc',
  'src/json/assetindex.cc',
  'src/json/client.cc',
  'src/entities/instance.cc',
  'src/config/config.cc',
  'src/config/java.cc',
  'src/entities/arch.cc',
  'src/entities/os.cc'
]

test_src = [
  'test/test.cc',
  'test/http.cc',
  'test/config/app.cc',
  'test/config/java.cc',
  'test/entities/arch.cc',
  'test/entities/os.cc',
  'test/json/client.cc'
]

# Build dependency
simdjson = dependency('simdjson', required: false)
if not simdjson.found()
  simdjson = cpp.find_library('simdjson', required: true)
endif

# Test dependency
gtest = dependency('gtest', required: false)

executable('SynthLauncher', 
  [src, 'src/main.cc'], 
  include_directories: 'include',
  dependencies: [simdjson, ws2_32, openssl, crypt32], 
  link_args: '-static'
)

test_exe = executable('SynthLauncherTest', [src, test_src], dependencies: [gtest, simdjson, ws2_32, openssl, crypt32])

test('SynthLauncherTest', test_exe)
