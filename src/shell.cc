#include "src/shell.hh"

#include <cstdio>
#include <cstdlib>
#include <stdexcept>

#include <errno.h>
#include <fcntl.h>
#include <sys/ioctl.h>
#include <termios.h>
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

    notifier_ =
        std::make_unique<QSocketNotifier>(pty_.value(), QSocketNotifier::Read);
    connect(notifier_.get(), &QSocketNotifier::activated, this,
            &ShellLauncher::outputReady);
    notifier_->setEnabled(true);

    is_running_ = true;
  } else {
    // Child
    pty_.reset();

    // termios slave_orig_term_settings;  // Saved terminal settings
    // termios new_term_settings;         // Current terminal settings
    // // Save the defaults parameters of the slave side of the PTY
    // int rc = tcgetattr(fds.value(), &slave_orig_term_settings);
    // // Set RAW mode on slave side of PTY
    // new_term_settings = slave_orig_term_settings;
    // // new_term_settings.c_cflag &= ~ECHO;
    // // cfmakeraw (&new_term_settings);
    // tcsetattr(fds.value(), TCSANOW, &new_term_settings);

    fds.dup(0);
    fds.dup(1);
    fds.dup(2);

    fds.reset();

    setsid();
    ioctl(0, TIOCSCTTY, 1);

    exec.run();
  }
}
