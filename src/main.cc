#include <cstdio>
#include <iostream>

#include <QApplication>
#include <QPushButton>
#include <QTextEdit>
#include <QVBoxLayout>
#include <QWidget>

#include "src/shell.hh"

class TextWidget : public QWidget {
 public:
  TextWidget() {
    layout_.addWidget(&shell_button_);
    layout_.addWidget(&send_button_);
    layout_.addWidget(&editor_);
    setLayout(&layout_);

    connect(&shell_button_, &QPushButton::clicked, this,
            &TextWidget::startShell);
    connect(&send_button_, &QPushButton::clicked, this, &TextWidget::sendInput);
  }

 private:
  void startShell() {
    shell_launcher_.Launch(Exec("/bin/bash", {"-i"}));
    connect(&shell_launcher_, &ShellLauncher::outputReady, this,
            &TextWidget::readyReadStdout);
    // connect(process_.get(),
    //         QOverload<int, QProcess::ExitStatus>::of(&QProcess::finished), this,
    //         &TextWidget::processFinished);

    // process_->setProgram("/bin/bash");
    // process_->setArguments({"-i"});
    // process_->start();
  }

  void sendInput() {
    shell_launcher_.pty()->writeAll("echo hello\n");
  }

  // void processFinished(int exitCode, QProcess::ExitStatus) {
  //   std::cerr << "proc finished: " << exitCode;
  // }

  void readyReadStdout() {
    QByteArray array;
    shell_launcher_.pty()->readAll(&array);
    QTextCursor cursor(editor_.document());
    cursor.movePosition(QTextCursor::End);
    cursor.insertText(array);
  }

  ShellLauncher shell_launcher_;
  QVBoxLayout layout_;
  QPushButton shell_button_{tr("&Shell")};
  QPushButton send_button_{tr("S&end")};
  QTextEdit editor_;
};

int main(int argc, char** argv) {
  QApplication app(argc, argv);
  TextWidget widget;
  widget.resize(1600, 1200);
  widget.show();
  app.exec();
}
