// Copyright 2019 Nicholas Bishop

#include "src/pane.hh"

Pane::Pane() {
  layout_.addWidget(&text_widget_);
  layout_.addWidget(&footer_);
  setLayout(&layout_);
}
