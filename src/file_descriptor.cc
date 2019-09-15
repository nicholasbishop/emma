// Copyright 2019 Nicholas Bishop

#include "src/file_descriptor.hh"

#include <cstdio>
#include <stdexcept>

#include <unistd.h>

FileDescriptor::FileDescriptor() : fd_(-1) {}

FileDescriptor::FileDescriptor(const int fd) : fd_(fd) {}

FileDescriptor::~FileDescriptor() {
  if (fd_ >= 0) {
    close(fd_);
  }
}

void FileDescriptor::operator=(int fd) {
  if (fd_ >= 0) {
    close(fd_);
  }
  fd_ = fd;
}

void FileDescriptor::reset() {
  if (fd_ >= 0) {
    close(fd_);
    fd_ = -1;
  }
}

bool FileDescriptor::isValid() const { return fd_ >= 0; }

int FileDescriptor::value() const { return fd_; };

void FileDescriptor::dup(int newfd) {
  if (dup2(fd_, newfd) == -1) {
    fprintf(stderr, "dup2 failed: %d\n", errno);
    throw std::runtime_error("dup2 failed");
  }
}

void FileDescriptor::readAll(QByteArray* array) {
  array->clear();
  for (;;) {
    const int chunk_size = 4096;
    const int previous_size = array->size();
    array->resize(array->size() + chunk_size);
    char* ptr = array->data() + previous_size;
    const int r = read(fd_, ptr, chunk_size);
    if (r == -1) {
      fprintf(stderr, "read failed: %d\n", errno);
      throw std::runtime_error("read failed");
    } else if (r < chunk_size) {
      array->resize(previous_size + r);
      break;
    }
  }
}

void FileDescriptor::writeAll(const QByteArray& array) {
  int remaining = array.size();
  const char* ptr = array.data();
  while (remaining > 0) {
    const int r = write(fd_, ptr, remaining);
    if (r == -1) {
      fprintf(stderr, "write failed: %d\n", errno);
      throw std::runtime_error("write failed");
    } else {
      ptr += r;
      remaining -= r;
    }
  }
}
