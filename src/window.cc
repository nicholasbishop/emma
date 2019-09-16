// Copyright 2019 Nicholas Bishop

#include "src/window.hh"

#include <QKeyEvent>

#include "src/pane.hh"

Column::Column() {
  layout_.addWidget(new Pane());
  setLayout(&layout_);
}

Pane* Column::firstPane() {
  return dynamic_cast<Pane*>(layout_.itemAt(0)->widget());
}

Pane* Column::getNextPane(Pane* pane) {
  int index = layout_.indexOf(pane);
  if (index < 0 || index + 1 == layout_.count()) {
    return nullptr;
  }
  return dynamic_cast<Pane*>(layout_.itemAt(index + 1)->widget());
}

Window::Window() {
  auto* column = new Column();
  active_pane_ = column->firstPane();
  layout_.addWidget(column);
  setLayout(&layout_);
}

void Window::keyPressEvent(QKeyEvent* event) {
  if (event->key() == Qt::Key_1 && event->modifiers() == Qt::ControlModifier) {
    addColumn();
  } else if (event->key() == Qt::Key_2 &&
             event->modifiers() == Qt::ControlModifier) {
    addRow();
  } else if (event->key() == Qt::Key_Tab &&
             event->modifiers() == Qt::ControlModifier) {
    activateNextPane();
  } else {
    QWidget::keyPressEvent(event);
  }
}

Column* Window::activeColumn() {
  return dynamic_cast<Column*>(active_pane_->parentWidget());
}

Column* Window::getNextColumnWithWraparound(Column* column) {
  int index = layout_.indexOf(column);
  if (index < 0 || index + 1 == layout_.count()) {
    index = 0;
  } else {
    index++;
  }
  return dynamic_cast<Column*>(layout_.itemAt(index)->widget());
}

void Window::addColumn() {
  auto* column = new Column();
  layout_.addWidget(column);
  setActivePane(column->firstPane());
}

void Window::addRow() {
  auto* column = activeColumn();
  auto* pane = new Pane();
  column->layout()->addWidget(pane);
  setActivePane(pane);
}

void Window::activateNextPane() {
  Column* column = activeColumn();
  Pane* pane = column->getNextPane(active_pane_);

  if (!pane) {
    column = getNextColumnWithWraparound(column);
    pane = column->firstPane();
  }
  setActivePane(pane);
}

void Window::setActivePane(Pane* pane) {
  pane->setFocus(Qt::OtherFocusReason);
  active_pane_ = pane;
}
