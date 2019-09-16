// Copyright 2019 Nicholas Bishop

#include "src/window.hh"

#include <QKeyEvent>

#include "src/pane.hh"

Window::Window() {
  active_pane_ = new Pane();
  layout_.addWidget(active_pane_);
  setLayout(&layout_);
}

void Window::keyPressEvent(QKeyEvent* event) {
  if (event->key() == Qt::Key_1 && event->modifiers() == Qt::ControlModifier) {
    splitVertical();
  } else if (event->key() == Qt::Key_2 && event->modifiers() == Qt::ControlModifier) {
    splitHorizontal();
  } else {
    QWidget::keyPressEvent(event);
  }
}

void Window::splitVertical() {
  auto* layout = dynamic_cast<QBoxLayout*>(active_pane_->parentWidget()->layout());
  if (layout->direction() == QBoxLayout::LeftToRight) {
    active_pane_ = new Pane();
    layout->addWidget(active_pane_);
  } else if (layout->count() == 1) {
    layout->setDirection(QBoxLayout::LeftToRight);
    active_pane_ = new Pane();
    layout->addWidget(active_pane_);
  } else {
    printf("TODO\n");
  }
}

void Window::splitHorizontal() {
}
