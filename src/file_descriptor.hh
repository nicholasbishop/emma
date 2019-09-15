// Copyright 2019 Nicholas Bishop

#ifndef SRC_FILE_DESCRIPTOR_HH_
#define SRC_FILE_DESCRIPTOR_HH_

#include <QByteArray>

class FileDescriptor {
 public:
  FileDescriptor();

  explicit FileDescriptor(int fd);

  ~FileDescriptor();

  void operator=(int fd);

  void reset();

  bool isValid() const;

  int value() const;

  void dup(int newfd);

  void readAll(QByteArray* array) ;

  void writeAll(const QByteArray& array);

 private:
  int fd_;
};

#endif  // SRC_FILE_DESCRIPTOR_HH_
