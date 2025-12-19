#include "MainWindow.hpp"
#include "NeoPDF.hpp"

#include <QMessageBox>
#include <QFileDialog>
#include <QFileInfo>
#include <QInputDialog>
#include <QtCharts/QChartView>
#include <QtCharts/QChart>
#include <QtCharts/QLineSeries>
#include <QtCharts/QAreaSeries>
#include <QtCharts/QValueAxis>
#include <QtCharts/QLogValueAxis>

#include <vector>
#include <numeric>
#include <cmath>

MainWindow::MainWindow(QWidget *parent) : QMainWindow(parent) {
    setupUI();
    setWindowTitle("NeoPDF Plotter");
    resize(1200, 800);
}

MainWindow::~MainWindow() {}

void MainWindow::setupUI() {
    centralWidget = new QWidget(this);
    setCentralWidget(centralWidget);

    mainLayout = new QHBoxLayout(centralWidget);

    // --- Controls Panel ---
    controlsLayout = new QVBoxLayout();

    // PDF Set Management
    setSelectionGroup = new QGroupBox("PDF Sets");
    setSelectionLayout = new QVBoxLayout();
    setListWidget = new QListWidget();
    addSetButton = new QPushButton("Add PDF Set");
    connect(addSetButton, &QPushButton::clicked, this, &MainWindow::onAddSetButtonClicked);
    connect(setListWidget, &QListWidget::currentItemChanged, this, &MainWindow::onCurrentSetChanged);

    setSelectionLayout->addWidget(setListWidget);
    setSelectionLayout->addWidget(addSetButton);
    setSelectionGroup->setLayout(setSelectionLayout);

    // Plotting Parameters
    plotParamsGroup = new QGroupBox("Plot Parameters");
    plotParamsLayout = new QFormLayout();

    xAxisVarCombo = new QComboBox();
    connect(xAxisVarCombo, QOverload<int>::of(&QComboBox::currentIndexChanged), this, &MainWindow::onXAxisVarChanged);

    pidCombo = new QComboBox();
    pidCombo->addItem("g (21)", 21);
    pidCombo->addItem("u (2)", 2);
    pidCombo->addItem("d (1)", 1);
    pidCombo->addItem("s (3)", 3);
    pidCombo->addItem("c (4)", 4);
    pidCombo->addItem("b (5)", 5);
    pidCombo->addItem("t (6)", 6);
    pidCombo->addItem("ubar (-2)", -2);
    pidCombo->addItem("dbar (-1)", -1);
    pidCombo->addItem("sbar (-3)", -3);
    pidCombo->addItem("cbar (-4)", -4);
    pidCombo->addItem("bbar (-5)", -5);
    pidCombo->addItem("tbar (-6)", -6);
    pidCombo->setCurrentIndex(0); // Default to gluon

    // Initialize all possible parameters
    m_paramInfos.append({NEOPDF_SUBGRID_PARAMS_NUCLEONS, "Nucleon (A)", nullptr, nullptr, false, 1.0, "1.0"});
    m_paramInfos.append({NEOPDF_SUBGRID_PARAMS_ALPHAS, "alpha_s", nullptr, nullptr, false, 0.118, "0.118"});
    m_paramInfos.append({NEOPDF_SUBGRID_PARAMS_XI, "xi", nullptr, nullptr, false, 0.0, "0.0"});
    m_paramInfos.append({NEOPDF_SUBGRID_PARAMS_DELTA, "delta", nullptr, nullptr, false, 0.0, "0.0"});
    m_paramInfos.append({NEOPDF_SUBGRID_PARAMS_KT, "kt", nullptr, nullptr, false, 0.0, "0.0"});
    m_paramInfos.append({NEOPDF_SUBGRID_PARAMS_MOMENTUM, "x", nullptr, nullptr, false, 0.1, "0.1"});
    m_paramInfos.append({NEOPDF_SUBGRID_PARAMS_SCALE, "Q2", nullptr, nullptr, false, 100.0, "100.0"});

    plotParamsLayout->addRow("X-axis variable:", xAxisVarCombo);
    plotParamsLayout->addRow("PID:", pidCombo);

    for (auto& info : m_paramInfos) {
        info.widget = new QLineEdit(info.default_text);
        info.label = new QLabel("Fixed " + info.name + " value:");
        plotParamsLayout->addRow(info.label, info.widget);
        info.widget->setVisible(false);
        info.label->setVisible(false);
    }

    rangeMinEdit = new QLineEdit("1e-5");
    rangeMaxEdit = new QLineEdit("1.0");
    pointsEdit = new QLineEdit("100");
    xAxisLogCheck = new QCheckBox("Logarithmic X-axis");
    yAxisLogCheck = new QCheckBox("Logarithmic Y-axis");

    plotParamsLayout->addRow("Plot Range Min:", rangeMinEdit);
    plotParamsLayout->addRow("Plot Range Max:", rangeMaxEdit);
    plotParamsLayout->addRow("Number of Points:", pointsEdit);
    plotParamsLayout->addRow(xAxisLogCheck);
    plotParamsLayout->addRow(yAxisLogCheck);

    plotParamsGroup->setLayout(plotParamsLayout);

    plotButton = new QPushButton("Plot");
    connect(plotButton, &QPushButton::clicked, this, &MainWindow::onPlotButtonClicked);

    controlsLayout->addWidget(setSelectionGroup);
    controlsLayout->addWidget(plotParamsGroup);
    controlsLayout->addWidget(plotButton);
    controlsLayout->addStretch();

    // --- Chart View ---
    chartView = new QChartView();
    chartView->setRenderHint(QPainter::Antialiasing);

    mainLayout->addLayout(controlsLayout, 1);
    mainLayout->addWidget(chartView, 3);
}

void MainWindow::onAddSetButtonClicked() {
    bool ok;
    QString setName = QInputDialog::getText(this, tr("Add PDF Set"),
                                             tr("PDF set name:"), QLineEdit::Normal,
                                             "", &ok);
    if (ok && !setName.isEmpty()) {
        QListWidgetItem* item = new QListWidgetItem(setName);
        item->setData(Qt::UserRole, setName);
        setListWidget->addItem(item);
        setListWidget->setCurrentItem(item);
    }
}

void MainWindow::onCurrentSetChanged(QListWidgetItem *current, QListWidgetItem *previous) {
    if (!current) return;
    updateParametersUI(current->data(Qt::UserRole).toString());
}

void MainWindow::updateParametersUI(const QString& setName) {
    neopdf::NeoPDF* pdf = nullptr;
    try {
        pdf = new neopdf::NeoPDF(setName.toStdString(), 0);
    } catch (const std::exception& e) {
        QMessageBox::critical(this, "Error loading PDF", e.what());
        // Reset UI to a default state
        for (auto& info : m_paramInfos) {
            info.active = false;
            info.widget->setVisible(false);
            info.label->setVisible(false);
        }
        xAxisVarCombo->clear();
        return;
    }

    xAxisVarCombo->blockSignals(true);
    xAxisVarCombo->clear();

    for (auto& info : m_paramInfos) {
        auto range = pdf->param_range(info.id);
        info.active = (range[0] < range[1]);
        info.widget->setVisible(info.active);
        info.label->setVisible(info.active);
        if (info.active) {
            xAxisVarCombo->addItem(info.name, QVariant::fromValue(info.id));
        }
    }

    delete pdf;
    xAxisVarCombo->blockSignals(false);
    onXAxisVarChanged(xAxisVarCombo->currentIndex());
}


void MainWindow::onXAxisVarChanged(int index) {
    if (index < 0) return;

    NeopdfSubgridParams selected_id = static_cast<NeopdfSubgridParams>(xAxisVarCombo->itemData(index).toInt());

    for (auto& info : m_paramInfos) {
        if (info.active) {
            info.widget->setEnabled(info.id != selected_id);
        }
    }
}

void MainWindow::onPlotButtonClicked() {
    if (setListWidget->selectedItems().isEmpty()) {
        QMessageBox::warning(this, "No PDF Set", "Please select a PDF set to plot.");
        return;
    }
    if (xAxisVarCombo->currentIndex() < 0) {
        QMessageBox::warning(this, "No variable selected", "Please select a variable to plot.");
        return;
    }

    // 1. Get parameters from UI
    QString setName = setListWidget->currentItem()->data(Qt::UserRole).toString();
    NeopdfSubgridParams xAxisVarId = static_cast<NeopdfSubgridParams>(xAxisVarCombo->currentData().toInt());

    bool ok;
    int pid = pidCombo->currentData().toInt();

    QMap<NeopdfSubgridParams, double> fixed_values;
    for (const auto& info : m_paramInfos) {
        if (info.active) {
            double val = info.widget->text().toDouble(&ok);
            if (!ok) {
                QMessageBox::warning(this, "Invalid Input", "Invalid value for " + info.name);
                return;
            }
            fixed_values[info.id] = val;
        }
    }

    double range_min = rangeMinEdit->text().toDouble(&ok);
    if (!ok) { QMessageBox::warning(this, "Invalid Input", "Invalid range min value."); return; }

    double range_max = rangeMaxEdit->text().toDouble(&ok);
    if (!ok) { QMessageBox::warning(this, "Invalid Input", "Invalid range max value."); return; }

    int n_points = pointsEdit->text().toInt(&ok);
    if (!ok || n_points <= 1) { QMessageBox::warning(this, "Invalid Input", "Number of points must be an integer greater than 1."); return; }

    bool isXLog = xAxisLogCheck->isChecked();
    if (isXLog && range_min <= 0.0) {
        QMessageBox::warning(this, "Invalid Input", "Minimum range for logarithmic X-axis must be positive.");
        return;
    }
    bool isYLog = yAxisLogCheck->isChecked();

    // 2. Load all members
    neopdf::NeoPDFs* pdfs = nullptr;
    try {
        pdfs = new neopdf::NeoPDFs(setName.toStdString());
    } catch (const std::exception& e) {
        QMessageBox::critical(this, "Error loading PDF", e.what());
        if (pdfs) delete pdfs;
        return;
    }

    auto *mean_series = new QLineSeries();
    mean_series->setName("Mean");
    auto *upper_series = new QLineSeries();
    auto *lower_series = new QLineSeries();

    // 3. Generate data points
    double step = isXLog ? std::pow(range_max / range_min, 1.0 / (n_points - 1)) : (range_max - range_min) / (n_points - 1);

    for (int i = 0; i < n_points; ++i) {
        double x_val = isXLog ? range_min * std::pow(step, i) : range_min + i * step;

        std::vector<double> params;
        for (const auto& info : m_paramInfos) {
            if (info.active) {
                if (info.id == xAxisVarId) {
                    params.push_back(x_val);
                } else {
                    params.push_back(fixed_values[info.id]);
                }
            }
        }

        std::vector<double> results_for_point;
        results_for_point.reserve(pdfs->size());
        for (size_t j = 0; j < pdfs->size(); ++j) {
            results_for_point.push_back(pdfs->at(j).xfxQ2_ND(pid, params));
        }

        double sum = std::accumulate(results_for_point.begin(), results_for_point.end(), 0.0);
        double mean = sum / results_for_point.size();

        double sq_sum = 0.0;
        for (const auto& val : results_for_point) {
            sq_sum += (val - mean) * (val - mean);
        }
        double std_dev = std::sqrt(sq_sum / results_for_point.size());

        mean_series->append(x_val, mean);
        upper_series->append(x_val, mean + std_dev);
        lower_series->append(x_val, mean - std_dev);
    }

    delete pdfs;

    auto *area_series = new QAreaSeries(upper_series, lower_series);
    area_series->setName("1-sigma Error Band");
    QPen pen(0x059669);
    pen.setWidth(2);
    mean_series->setPen(pen);
    area_series->setColor(QColor(0x6EE7B7));
    area_series->setBorderColor(QColor(0x6EE7B7));

    auto *chart = new QChart();
    chart->addSeries(area_series);
    chart->addSeries(mean_series);
    chart->setTitle("PDF: " + setName + " (pid=" + QString::number(pid) + ")");

    QAbstractAxis *axisX;
    if (isXLog) {
        auto *logAxis = new QLogValueAxis();
        logAxis->setBase(10.0);
        logAxis->setLabelFormat("%.0e");
        logAxis->setMinorTickCount(-1);
        axisX = logAxis;
    } else {
        auto *valAxis = new QValueAxis();
        valAxis->setLabelFormat("%.1e");
        axisX = valAxis;
    }
    axisX->setTitleText(xAxisVarCombo->currentText());
    chart->addAxis(axisX, Qt::AlignBottom);
    mean_series->attachAxis(axisX);
    area_series->attachAxis(axisX);

    QAbstractAxis *axisY;
    if (isYLog) {
        auto *logAxis = new QLogValueAxis();
        logAxis->setBase(10.0);
        logAxis->setLabelFormat("%.0e");
        logAxis->setMinorTickCount(-1);
        axisY = logAxis;
    } else {
        axisY = new QValueAxis();
    }

    QString yTitle = "x * f(...)";
    axisY->setTitleText(yTitle);
    chart->addAxis(axisY, Qt::AlignLeft);
    mean_series->attachAxis(axisY);
    area_series->attachAxis(axisY);

    chart->legend()->setVisible(true);
    chart->legend()->setAlignment(Qt::AlignBottom);

    chartView->setChart(chart);
}
