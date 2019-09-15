// Copyright 2019 Nicholas Bishop

#include "src/exec.hh"

#include <cstring>
#include <stdexcept>

#include <unistd.h>

Exec::Exec(const std::string& path, const std::vector<std::string>& args) : path_(path), args_(args) {}

static char* copy_string(const std::string& str) {
  char* copy = new char[str.size() + 1];
  memcpy(copy, str.data(), str.size());
  copy[str.size()] = '\0';
  return copy;
}

void Exec::run() const {
  std::vector<char*> argv;
  argv.reserve(args_.size() + 2);
  argv.push_back(copy_string(path_));
  for (const auto& arg : args_) {
    argv.push_back(copy_string(arg));
  }
  argv.push_back(nullptr);
  const int r = execv(path_.c_str(), argv.data());
  for (auto* s : argv) {
    delete s;
  }
  if (r) {
    fprintf(stderr, "execv failed: %d\n", errno);
    throw std::runtime_error("execv failed");
  }
}
