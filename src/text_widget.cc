// Copyright 2019 Nicholas Bishop

#include "src/text_widget.hh"

#include <QKeyEvent>

TextWidget::TextWidget() {
  shell_cursor_.setKeepPositionOnInsert(true);
}

void TextWidget::keyPressEvent(QKeyEvent* event) {
  if (event->key() == Qt::Key_S && event->modifiers() == Qt::AltModifier) {
    startShell();
  } else if (event->key() == Qt::Key_Return && shell_launcher_.isRunning()) {
    runCommand();
  } else {
    QTextEdit::keyPressEvent(event);
  }
}

void TextWidget::startShell() {
  if (shell_launcher_.isRunning()) {
    return;
  }

  shell_launcher_.Launch(Exec("/bin/bash", {"-i"}));
  connect(&shell_launcher_, &ShellLauncher::outputReady, this,
          &TextWidget::shellReadReady);
  // connect(process_.get(),
  //         QOverload<int, QProcess::ExitStatus>::of(&QProcess::finished),
  //         this, &TextWidget::processFinished);

  // process_->setProgram("/bin/bash");
  // process_->setArguments({"-i"});
  // process_->start();
}

void TextWidget::runCommand() {
  // If the cursor position is before the shell prompt, do
  // nothing. TODO(nicholasbishop): this should actually pass the
  // return-key press through to the regular text editor, not swallow
  // the action.
  if (shell_cursor_.position() > textCursor().position()) {
    return;
  }

  QTextCursor sel(document());
  sel.setPosition(shell_cursor_.position());
  sel.movePosition(QTextCursor::End, QTextCursor::KeepAnchor);
  const QString text(sel.selectedText() + "\n");

  QTextCursor newline_cursor(document());
  newline_cursor.movePosition(QTextCursor::End);
  newline_cursor.insertText("\n");

  shell_cursor_.movePosition(QTextCursor::End);

  printf("sending text: '%s'\n", text.toStdString().c_str());
  shell_launcher_.pty()->writeAll(text.toUtf8());
}

// void TextWidget::sendInput() { shell_launcher_.pty()->writeAll("echo hello\n"); }

// void processFinished(int exitCode, QProcess::ExitStatus) {
//   std::cerr << "proc finished: " << exitCode;
// }

void TextWidget::shellReadReady() {
  QByteArray array;
  shell_launcher_.pty()->readAll(&array);

  QString text = array;
  QString crlf;
  crlf += QChar(13);
  crlf += QChar(10);
  text.replace(crlf, "\n");
  //text.replace(QChar(13), '2');
  for (const QChar c : text) {
    //text = text.replace('\n', "_");
    printf("%s (%d)\n", QString(c).toUtf8().toStdString().c_str(), c.unicode());
  }

  //printf("inserting text: '%s'\n", text.toStdString().c_str());

  appendShellOutput(text);
}

void TextWidget::appendShellOutput(const QString& text) {
  shell_cursor_.setKeepPositionOnInsert(false);
  shell_cursor_.insertText(text);
  shell_cursor_.setKeepPositionOnInsert(true);
}
