// Copyright 2019 Nicholas Bishop

#ifndef SRC_TEXT_WIDGET_HH_
#define SRC_TEXT_WIDGET_HH_

#include <QTextEdit>

#include "src/shell.hh"

class TextWidget : public QTextEdit {
 public:
  TextWidget();

 protected:
  void keyPressEvent(QKeyEvent* event) final;

 private:
  void startShell();
  void shellReadReady();
  void runCommand();
  void appendShellOutput(const QString& text);

  ShellLauncher shell_launcher_;
  QTextCursor shell_cursor_{document()};
};

#endif  // SRC_TEXT_WIDGET_HH_
