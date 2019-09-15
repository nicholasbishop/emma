// Copyright 2019 Nicholas Bishop

#ifndef SRC_PANE_HH_
#define SRC_PANE_HH_

#include <QLabel>
#include <QVBoxLayout>
#include <QWidget>

#include "src/text_widget.hh"

class Pane : public QWidget {
 public:
  Pane();

 private:
  QVBoxLayout layout_;
  TextWidget text_widget_;
  QLabel footer_;
};

#endif  // SRC_PANE_HH_
