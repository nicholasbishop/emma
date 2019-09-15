// Copyright 2019 Nicholas Bishop

#ifndef SRC_WINDOW_HH_
#define SRC_WINDOW_HH_

#include <QBoxLayout>
#include <QWidget>

class Window : public QWidget {
 public:
  Window();

 private:
  QBoxLayout layout_{QBoxLayout::LeftToRight};
};

#endif  // SRC_WINDOW_HH_
