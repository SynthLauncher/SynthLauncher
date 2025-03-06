project('SynthLauncher', 'cpp', license: 'aGPL-3.0', version: '1.0.0', license_files: 'LICENSE', default_options: ['cpp_std=c++17', 'warning_level=3'])

cpp = meson.get_compiler('cpp')

# Platform-specific dependencies
if host_machine.system() == 'windows'
  ws2_32 = cpp.find_library('ws2_32')
  crypt32 = cpp.find_library('crypt32')
  minizip = cpp.find_library('minizip', required: true)
  platform_deps = [ws2_32, crypt32, minizip]
elif host_machine.system() == 'darwin'
  minizip = cpp.find_library('minizip', required: true)
  platform_deps = [minizip]
else
  minizip = cpp.find_library('minizip', required: true)
  platform_deps = [minizip]
endif

openssl = dependency('openssl')

src = [
  'src/json/client.cc',
  'src/json/manifest.cc',
  'src/json/assetindex.cc',
  'src/config/app.cc',
  'src/config/java.cc',
  'src/config/config.cc',
  'src/entities/os.cc',
  'src/entities/arch.cc',
  'src/entities/instance.cc',
]

test_src = [
  'test/test.cc',
  'test/utils/rapidjson_utils.cc',
  'test/utils/httplib_utils.cc',
  'test/json/client.cc',
  'test/json/manifest.cc',
  'test/json/assetindex.cc',
  'test/config/app.cc',
  'test/config/java.cc',
  'test/entities/instance.cc'
]

# Test dependency
gtest = dependency('gtest', required: false)
if not gtest.found()
  gtest = cpp.find_library('gtest', required: true)
endif

executable('SynthLauncher', 
  [src, 'src/main.cc'], 
  include_directories: 'include',
  dependencies: [openssl] + platform_deps, 
)

test_exe = executable('SynthLauncherTest', [src, test_src], dependencies: [gtest, openssl] + platform_deps)

test('SynthLauncherTest', test_exe)