#include "include/entities/instance.hh"

fs::path Instance::PARENT_DIR;
fs::path Instance::INSTANCE_FILE;

Instance::Instance(const std::string& name, const std::string& version) : name(name), version(version) {}

void Instance::init(AppConfig &config) {
  PARENT_DIR = config.DIR + "instances/";
  INSTANCE_FILE = config.DIR + "instances.json";
}

fs::path Instance::dir() { 
    return this->PARENT_DIR / this->name; 
}

void Instance::initDir() {
    if (!fs::exists(this->dir()))
      fs::create_directory(this->dir());
}

Instance Instance::createInstance(const std::string& name, const std::string& version) {
  Instance instance = Instance(name, version);

  Manifest manifest = Manifest::parse();

  std::string url = "";

  for (Manifest::Version manifest_version : manifest.versions) {
    if (manifest_version.id == version) {
      url = manifest_version.url;
        
      break;
    }
  }

  if (url == "")
    throw std::runtime_error(
        "Unexpected version occured while creating an instance!");

  instance.initDir();
  
  fs::path client_path = instance.dir() / "client.json";
  
  /*
      I've got more to implement here!
  */
}