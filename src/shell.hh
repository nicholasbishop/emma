// Copyright 2019 Nicholas Bishop

#ifndef SRC_SHELL_HH_
#define SRC_SHELL_HH_

#include <memory>

#include <QObject>
#include <QSocketNotifier>

#include "src/exec.hh"
#include "src/file_descriptor.hh"

class ShellLauncher : public QObject {
  Q_OBJECT

 public:
  void Launch(const Exec& exec);

  bool isRunning() const { return is_running_; }

  FileDescriptor* pty() { return &pty_; }

 signals:
  void outputReady();

 private:
  std::unique_ptr<QSocketNotifier> notifier_;
  FileDescriptor pty_;
  bool is_running_{false};
};

#endif  // SRC_SHELL_HH_
