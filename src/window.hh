// Copyright 2019 Nicholas Bishop

#ifndef SRC_WINDOW_HH_
#define SRC_WINDOW_HH_

#include <QBoxLayout>
#include <QWidget>

class Pane;

class Column : public QWidget {
 public:
  Column();

  Pane* firstPane();

  Pane* getNextPane(Pane* pane);

 private:
  QVBoxLayout layout_;
};

class Window : public QWidget {
 public:
  Window();

 protected:
  void keyPressEvent(QKeyEvent* event) final;

 private:
  Column* activeColumn();
  Column* getNextColumnWithWraparound(Column* column);

  void addColumn();
  void addRow();
  void activateNextPane();
  void setActivePane(Pane* pane);

  Pane* active_pane_ = nullptr;
  QHBoxLayout layout_;
};

#endif  // SRC_WINDOW_HH_
