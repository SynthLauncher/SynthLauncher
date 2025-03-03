project('SynthLauncher', 'cpp', license: 'aGPL-3.0', version: '1.0.0', license_files: 'LICENSE', default_options: ['cpp_std=c++20', 'warning_level=3'])

cpp = meson.get_compiler('cpp')

# Platform-specific dependencies
if host_machine.system() == 'windows'
  ws2_32 = cpp.find_library('ws2_32')
  crypt32 = cpp.find_library('crypt32')
  platform_deps = [ws2_32, crypt32]
elif host_machine.system() == 'darwin'
  platform_deps = []
else
  platform_deps = []
endif

openssl = dependency('openssl')

src = [
  'src/json/manifest.cc',
  'src/config/app.cc',
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
  'test/config/app.cc',
  'test/config/java.cc',
  'test/entities/arch.cc',
  'test/entities/os.cc',
  'test/json/client.cc',
  'test/json/manifest.cc'
]

# Build dependency
simdjson = dependency('simdjson', required: false)
if not simdjson.found()
  simdjson = cpp.find_library('simdjson', required: true)
endif

# Test dependency
gtest = dependency('gtest', required: false)
if not gtest.found()
  gtest = cpp.find_library('gtest', required: false)
endif

executable('SynthLauncher', 
  [src, 'src/main.cc'], 
  include_directories: 'include',
  dependencies: [simdjson, openssl] + platform_deps, 
  link_args: '-static'
)

test_exe = executable('SynthLauncherTest', [src, test_src], dependencies: [gtest, simdjson, openssl] + platform_deps)

test('SynthLauncherTest', test_exe)