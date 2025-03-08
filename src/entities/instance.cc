#include "include/entities/instance.hh"

Instance::Instance() : name(""), version("") {}

Instance::Instance(const std::string_view name, const std::string_view version)
    : name(name), version(version) {}

Instance Instance::fromJson(const rapidjson::Value &obj) {
  if (!obj.HasMember("name") || !obj.HasMember("version"))
    return Instance();

  return Instance(obj["name"].GetString(), obj["version"].GetString());
}

std::string Instance::toJson() const {
  std::ostringstream json;
  json << "{"
       << "\"name\" \"" << this->name << "\","
       << "\"version\": \"" << this->version << "\""
       << "}";

  return json.str();
}

void Instance::init(const App::AppConfig &config) {
  PARENT_DIR = config.DIR / "instances";
  INSTANCE_FILE = config.DIR / "instances.json";
}

fs::path Instance::dir() { return PARENT_DIR / name; }

void Instance::initDir() { fs::create_directories(this->dir()); }

Instance Instance::createInstance(const std::string_view name, const std::string_view version) {
  Instance instance = Instance(name, version);
  Manifest manifest = Manifest::fromJson();
  std::string url = "";

  for (Manifest::Version manifest_version : manifest.versions) {
    if (manifest_version.id == version) {
      url = manifest_version.url;
      break;
    }
  }

  if (url.empty()) {
    throw std::runtime_error("Version '" + std::string(version) +
                             "' not found in manifest!");
  }

  instance.initDir();
  fs::path client_path = instance.dir() / "client.json";

  if (fs::exists(client_path)) {
    throw std::runtime_error("Instance already exists at: " +
                             client_path.string());
  }

  auto [host, path] = httplib_utils::extractHostAndPath(url);

  httplib::Client cli(host);

  if (auto res = cli.Get(path)) {
    if (res->status == 200) {
      std::ofstream outFile(client_path);
      if (!outFile)
        throw std::runtime_error("Failed to open " + client_path.string());

      outFile << res->body;
    } else {
      throw std::runtime_error("Unexpected HTTP status: " +
                               std::to_string(res->status));
    }
  } else {
    auto err = res.error();
    throw std::runtime_error("HTTP request failed: " + httplib::to_string(err));
  }

  return instance;
}

std::vector<Instance> Instance::readInstances() {
  std::vector<Instance> instances;

  auto json = rapidjson_utils::fromJson(INSTANCE_FILE);

  for (const auto &instance : json.GetArray())
    instances.push_back(Instance::fromJson(instance));

  return instances;
}

Instance Instance::getInstance(const std::string &name) {
  std::vector<Instance> instances = readInstances();

  for (Instance instance : instances) {
    if (instance.name == name)
      return instance;
  }

  throw std::runtime_error("Instance not found!");
}

void Instance::writeInstance(Instance &instance) {
  std::string json = instance.toJson();

  std::ofstream file(INSTANCE_FILE);
  file.write(json.c_str(), sizeof(json));
}

void Instance::updateInstance(Instance &instance) {
  /*
    I'll implement this later :)
  */
}

void Instance::addInstance(Instance &instance) {
  /*
    I'll implement this later :)
  */
}

Config Instance::getConfig() {
  fs::path path = this->dir() / "config.json";
  return Config::getConfig(path);
}

Client Instance::readClient() {
  fs::path path = this->dir() / "client.json";
  auto json = rapidjson_utils::fromJson(path);

  return Client::fromJson(json);
}

void Instance::install(App::AppConfig &config) {
  this->readClient().download(config, this->dir());
}

void Instance::launch(App::AppConfig &config) {
  this->getConfig().launch(config, *this);
}