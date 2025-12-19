#pragma once

#include <QMainWindow>
#include <QtWidgets>
#include <QtCharts/QChartView>

class MainWindow : public QMainWindow {
    Q_OBJECT

public:
    explicit MainWindow(QWidget *parent = nullptr);
    ~MainWindow();

private slots:
    void onPlotButtonClicked();
    void onAddSetButtonClicked();
    void onXAxisVarChanged(int index);

private:
    void setupUI();

    // Main layout
    QWidget *centralWidget;
    QHBoxLayout *mainLayout;
    QVBoxLayout *controlsLayout;

    // Controls
    QGroupBox *setSelectionGroup;
    QVBoxLayout *setSelectionLayout;
    QListWidget *setListWidget;
    QPushButton *addSetButton;

    QGroupBox *plotParamsGroup;
    QFormLayout *plotParamsLayout;
    QComboBox *xAxisVarCombo;
    QComboBox *pidCombo;
    QLineEdit *q2ValueEdit;
    QLineEdit *xValueEdit;
    QLineEdit *rangeMinEdit;
    QLineEdit *rangeMaxEdit;
    QLineEdit *pointsEdit;
    QCheckBox *xAxisLogCheck;
    QCheckBox *yAxisLogCheck;
    QPushButton *plotButton;

    // Plotting
    QChartView *chartView;
};
