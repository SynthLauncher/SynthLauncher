project('SynthLauncher', 'cpp', license: 'aGPL-3.0', version: '1.0.0', license_files: 'LICENSE', default_options: ['cpp_std=c++17', 'warning_level=3'])

src = [
  'src/entities/os.cc',
  'src/entities/arch.cc',
  'src/json/manifest.cc',
  'src/config/app.cc',
  'src/http/builder.cc',
  'src/http/parser.cc',
  'src/json/assetindex.cc',
  'src/json/client.cc',
  'src/entities/instance.cc',
  'src/config/config.cc',
  'src/config/java.cc'
]

test_src = [
  'test/test.cc',
  'test/http.cc',
  'test/dir.cc'
]

# Build dependency
simdjson = dependency('simdjson', required: false)

# Test dependency
gtest = dependency('gtest', required: false)

executable('SynthLauncher', [src, 'src/main.cc'], include_directories: 'include', dependencies: [simdjson])

test_exe = executable('SynthLauncherTest', [src, test_src], dependencies: [gtest, simdjson])

test('SynthLauncherTest', test_exe)
