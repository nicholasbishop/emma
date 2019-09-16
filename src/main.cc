#include <cstdio>
#include <iostream>

#include <QApplication>

#include "src/window.hh"

int main(int argc, char** argv) {
  QApplication app(argc, argv);
  Window window;
  window.resize(2400, 1200);
  window.show();
  app.exec();
}
