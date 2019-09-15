// Copyright 2019 Nicholas Bishop

#include "src/pane.hh"
#include "src/window.hh"

Window::Window() {
  layout_.addWidget(new Pane());
  setLayout(&layout_);
}
