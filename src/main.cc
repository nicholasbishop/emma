#include <cstdio>
#include <iostream>

#include <QApplication>

#include "src/text_widget.hh"

int main(int argc, char** argv) {
  QApplication app(argc, argv);
  TextWidget widget;
  widget.resize(1600, 1200);
  widget.show();
  app.exec();
}
