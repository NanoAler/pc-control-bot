#include <QtCore/QCoreApplication>
#include <QtCore/QDateTime>
#include <QtCore/QString>
#include <QtGui/QGuiApplication>
#include <QtGui/QScreen>
#include <QtGui/QPixmap>
#include <QtCore/QDebug>

int main(int argc, char *argv[]) {
    qputenv("QT_QPA_PLATFORM", QByteArray("xcb"));
    QCoreApplication::setAttribute(Qt::AA_ShareOpenGLContexts);
    QGuiApplication app(argc, argv);

    QString outputPath = QString("/tmp/screenshot_%1.png")
        .arg(QDateTime::currentDateTime().toString("yyyyMMdd_hhmmss"));

    QList<QScreen*> screens = app.screens();
    if (screens.isEmpty()) {
        qWarning() << "No screens found";
        return 1;
    }

    QScreen *screen = screens.first();
    QPixmap pixmap = screen->grabWindow(0);

    if (pixmap.isNull() || !pixmap.save(outputPath, "PNG")) {
        qWarning() << "Failed to capture screenshot";
        return 1;
    }

    qInfo() << "OK:" << outputPath;
    return 0;
}
