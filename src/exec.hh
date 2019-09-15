// Copyright 2019 Nicholas Bishop

#ifndef SRC_EXEC_HH_
#define SRC_EXEC_HH_

#include <string>
#include <vector>

class Exec {
 public:
  Exec(const std::string& path, const std::vector<std::string>& args);

  void run() const;

 private:
  std::string path_;
  std::vector<std::string> args_;
};

#endif  // SRC_EXEC_HH_
