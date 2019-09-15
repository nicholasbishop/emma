#include "src/shell.hh"

#include <cstdlib>
#include <cstdio>
#include <stdexcept>

#include <errno.h>
#include <fcntl.h>
#include <sys/ioctl.h>
#include <unistd.h>

#include "src/file_descriptor.hh"

void ShellLauncher::Launch(const Exec& exec) {
  pty_ = posix_openpt(O_RDWR);
  if (!pty_.isValid()) {
    fprintf(stderr, "Error %d on posix_openpt()\n", errno);
    throw std::runtime_error("posix_openpt failed");
  }

  if (grantpt(pty_.value())) {
    fprintf(stderr, "Error %d on grantpt()\n", errno);
    throw std::runtime_error("grantpt failed");
  }

  if (unlockpt(pty_.value())) {
    fprintf(stderr, "Error %d on unlockpt()\n", errno);
    throw std::runtime_error("unlockpt failed");
  }

  FileDescriptor fds(open(ptsname(pty_.value()), O_RDWR));
  if (!fds.isValid()) {
    fprintf(stderr, "Error %d opening pty child\n", errno);
    throw std::runtime_error("child pty open failed");
  }

  if (fork()) {
    // Parent
    fds.reset();

    notifier_ = std::make_unique<QSocketNotifier>(pty_.value(),
                                                  QSocketNotifier::Read);
    connect(notifier_.get(), &QSocketNotifier::activated,
            this, &ShellLauncher::outputReady);
    notifier_->setEnabled(true);
  } else {
    // Child
    pty_.reset();

    fds.dup(0);
    fds.dup(1);
    fds.dup(2);

    fds.reset();

    setsid();
    ioctl(0, TIOCSCTTY, 1);

    exec.run();
  }
}
