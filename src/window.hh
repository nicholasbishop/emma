// Copyright 2019 Nicholas Bishop

#ifndef SRC_WINDOW_HH_
#define SRC_WINDOW_HH_

#include <QBoxLayout>
#include <QWidget>

class Pane;

class Window : public QWidget {
 public:
  Window();

 protected:
  void keyPressEvent(QKeyEvent* event) final;

 private:
  void splitVertical();
  void splitHorizontal();

  Pane* active_pane_ = nullptr;
  QBoxLayout layout_{QBoxLayout::LeftToRight};
};

#endif  // SRC_WINDOW_HH_
