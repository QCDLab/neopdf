#pragma once

#include <QMainWindow>
#include <QtWidgets>
#include <QtCharts/QChartView>
#include <QVector>

#include "neopdf_capi.h" // For NeopdfSubgridParams enum

// Forward declarations
class QFormLayout;
class QLabel;
class QListWidgetItem;
class QLineEdit;

struct ParamInfo {
    NeopdfSubgridParams id;
    QString name;
    QLineEdit* widget = nullptr;
    QLabel* label = nullptr;
    bool active = false;
    double default_val = 0.0;
    QString default_text;
};

class MainWindow : public QMainWindow {
    Q_OBJECT

public:
    explicit MainWindow(QWidget *parent = nullptr);
    ~MainWindow();

private slots:
    void onPlotButtonClicked();
    void onAddSetButtonClicked();
    void onXAxisVarChanged(int index);
    void onSelectionSetChanged();
    void onClearSetsButtonClicked();

private:
    void setupUI();
    void updateParametersUI(const QList<QListWidgetItem*>& items);

    // Main layout
    QWidget *centralWidget;
    QHBoxLayout *mainLayout;
    QVBoxLayout *controlsLayout;

    // Controls
    QGroupBox *setSelectionGroup;
    QVBoxLayout *setSelectionLayout;
    QListWidget *setListWidget;
    QPushButton *addSetButton;
    QPushButton *clearSetsButton;

    QGroupBox *plotParamsGroup;
    QFormLayout *plotParamsLayout;
    QComboBox *xAxisVarCombo;
    QComboBox *pidCombo;

    QVector<ParamInfo> m_paramInfos;

    QLineEdit* rangeMinEdit;
    QLineEdit* rangeMaxEdit;
    QLineEdit* pointsEdit;
    QCheckBox *xAxisLogCheck;
    QCheckBox *yAxisLogCheck;
    QPushButton *plotButton;

    // Plotting
    QChartView *chartView;
};
