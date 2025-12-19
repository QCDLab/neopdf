#include <QFileDialog>
#include <QFileInfo>
#include <QInputDialog>
#include <QMessageBox>
#include <QSet>
#include <QtCharts/QAreaSeries>
#include <QtCharts/QChart>
#include <QtCharts/QChartView>
#include <QtCharts/QLegendMarker>
#include <QtCharts/QLineSeries>
#include <QtCharts/QLogValueAxis>
#include <QtCharts/QValueAxis>

#include <cmath>
#include <numeric>
#include <vector>

#include "MainWindow.hpp"
#include "NeoPDF.hpp"

MainWindow::MainWindow(QWidget *parent) : QMainWindow(parent) {
    setupUI();
    setWindowTitle("NeoPDF");
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
    setListWidget->setSelectionMode(QAbstractItemView::ExtendedSelection);
    addSetButton = new QPushButton("Add Set");
    connect(addSetButton, &QPushButton::clicked, this,
            &MainWindow::onAddSetButtonClicked);
    connect(setListWidget->selectionModel(),
            &QItemSelectionModel::selectionChanged, this,
            &MainWindow::onSelectionSetChanged);

    setSelectionLayout->addWidget(setListWidget);
    setSelectionLayout->addWidget(addSetButton);
    clearSetsButton = new QPushButton("Clear All");
    setSelectionLayout->addWidget(clearSetsButton);
    connect(clearSetsButton, &QPushButton::clicked, this,
            &MainWindow::onClearSetsButtonClicked);
    setSelectionGroup->setLayout(setSelectionLayout);

    // Plotting Parameters
    plotParamsGroup = new QGroupBox("Plot Parameters");
    plotParamsLayout = new QFormLayout();

    xAxisVarCombo = new QComboBox();
    connect(xAxisVarCombo, QOverload<int>::of(&QComboBox::currentIndexChanged),
            this, &MainWindow::onXAxisVarChanged);

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

    m_paramInfos.append({NEOPDF_SUBGRID_PARAMS_NUCLEONS, "Nucleon (A)", nullptr,
                         nullptr, false, 1.0, "1.0"});
    m_paramInfos.append({NEOPDF_SUBGRID_PARAMS_ALPHAS, "alpha_s", nullptr,
                         nullptr, false, 0.118, "0.118"});
    m_paramInfos.append(
        {NEOPDF_SUBGRID_PARAMS_XI, "xi", nullptr, nullptr, false, 0.0, "0.0"});
    m_paramInfos.append({NEOPDF_SUBGRID_PARAMS_DELTA, "delta", nullptr, nullptr,
                         false, 0.0, "0.0"});
    m_paramInfos.append(
        {NEOPDF_SUBGRID_PARAMS_KT, "kt", nullptr, nullptr, false, 0.0, "0.0"});
    m_paramInfos.append({NEOPDF_SUBGRID_PARAMS_MOMENTUM, "x", nullptr, nullptr,
                         false, 0.1, "0.1"});
    m_paramInfos.append({NEOPDF_SUBGRID_PARAMS_SCALE, "Q2", nullptr, nullptr,
                         false, 100.0, "100.0"});

    plotParamsLayout->addRow("X-axis variable:", xAxisVarCombo);
    plotParamsLayout->addRow("PID:", pidCombo);

    for (auto &info : m_paramInfos) {
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
    connect(plotButton, &QPushButton::clicked, this,
            &MainWindow::onPlotButtonClicked);

    controlsLayout->addWidget(setSelectionGroup);
    controlsLayout->addWidget(plotParamsGroup);
    controlsLayout->addWidget(plotButton);
    controlsLayout->addStretch();

    chartView = new QChartView();
    chartView->setRenderHint(QPainter::Antialiasing);

    mainLayout->addLayout(controlsLayout, 1);
    mainLayout->addWidget(chartView, 3);
}

void MainWindow::onAddSetButtonClicked() {
    bool ok;
    QString setName =
        QInputDialog::getText(this, tr("Add PDF Set"), tr("PDF set name:"),
                              QLineEdit::Normal, "", &ok);
    if (ok && !setName.isEmpty()) {
        QListWidgetItem *item = new QListWidgetItem(setName);
        item->setData(Qt::UserRole, setName);
        setListWidget->addItem(item);
        setListWidget->setCurrentItem(item);
    }
}

void MainWindow::onSelectionSetChanged() {
    updateParametersUI(setListWidget->selectedItems());
}

void MainWindow::updateParametersUI(const QList<QListWidgetItem *> &items) {
    if (items.isEmpty()) {
        for (auto &info : m_paramInfos) {
            info.active = false;
            info.widget->setVisible(false);
            info.label->setVisible(false);
        }
        xAxisVarCombo->clear();
        return;
    }

    QSet<NeopdfSubgridParams> commonParams;
    bool first = true;

    for (const auto &item : items) {
        QSet<NeopdfSubgridParams> itemParams;
        try {
            neopdf::NeoPDF pdf(
                item->data(Qt::UserRole).toString().toStdString(), 0);
            for (const auto &info : m_paramInfos) {
                auto range = pdf.param_range(info.id);
                if (range[0] < range[1]) {
                    itemParams.insert(info.id);
                }
            }
        } catch (const std::exception &e) {
            QMessageBox::warning(this, "Error Loading Set",
                                 "Could not inspect " + item->text() + ":\n" +
                                     e.what());
            continue;
        }

        if (first) {
            commonParams = itemParams;
            first = false;
        } else {
            commonParams.intersect(itemParams);
        }
    }

    xAxisVarCombo->blockSignals(true);
    xAxisVarCombo->clear();

    for (auto &info : m_paramInfos) {
        info.active = commonParams.contains(info.id);
        info.widget->setVisible(info.active);
        info.label->setVisible(info.active);
        if (info.active) {
            xAxisVarCombo->addItem(info.name, static_cast<int>(info.id));
        }
    }

    xAxisVarCombo->blockSignals(false);
    onXAxisVarChanged(xAxisVarCombo->currentIndex());
}

void MainWindow::onXAxisVarChanged(int index) {
    if (index < 0)
        return;

    auto selected_id = static_cast<NeopdfSubgridParams>(
        xAxisVarCombo->itemData(index).toInt());

    for (auto &info : m_paramInfos) {
        if (info.active) {
            info.widget->setEnabled(info.id != selected_id);
        }
    }
}

void MainWindow::onClearSetsButtonClicked() {
    setListWidget->clear();
    updateParametersUI({}); // Call with empty list to reset UI
}

void MainWindow::onPlotButtonClicked() {
    QList<QListWidgetItem *> selectedItems = setListWidget->selectedItems();
    if (selectedItems.isEmpty()) {
        QMessageBox::warning(this, "No PDF Set",
                             "Please select one or more PDF sets to plot.");
        return;
    }
    if (xAxisVarCombo->currentIndex() < 0) {
        QMessageBox::warning(this, "No variable selected",
                             "No common variables to plot. Please select "
                             "sets with compatible kinematics.");
        return;
    }

    bool ok;
    auto xAxisVarId =
        static_cast<NeopdfSubgridParams>(xAxisVarCombo->currentData().toInt());
    int pid = pidCombo->currentData().toInt();

    QMap<NeopdfSubgridParams, double> fixed_values;
    for (const auto &info : m_paramInfos) {
        if (info.active && info.id != xAxisVarId) {
            double val = info.widget->text().toDouble(&ok);
            if (!ok) {
                QMessageBox::warning(this, "Invalid Input",
                                     "Invalid value for " + info.name);
                return;
            }
            fixed_values[info.id] = val;
        }
    }

    double range_min = rangeMinEdit->text().toDouble(&ok);
    if (!ok) {
        QMessageBox::warning(this, "Invalid Input", "Invalid range min value.");
        return;
    }
    double range_max = rangeMaxEdit->text().toDouble(&ok);
    if (!ok) {
        QMessageBox::warning(this, "Invalid Input", "Invalid range max value.");
        return;
    }
    int n_points = pointsEdit->text().toInt(&ok);
    if (!ok || n_points <= 1) {
        QMessageBox::warning(this, "Invalid Input",
                             "Number of points must be an integer > 1.");
        return;
    }

    bool isXLog = xAxisLogCheck->isChecked();
    if (isXLog && range_min <= 0.0) {
        QMessageBox::warning(
            this, "Invalid Input",
            "Minimum range for logarithmic X-axis must be positive.");
        return;
    }
    bool isYLog = yAxisLogCheck->isChecked();

    auto *chart = new QChart();
    auto *y_axis = isYLog ? static_cast<QAbstractAxis *>(new QLogValueAxis())
                          : static_cast<QAbstractAxis *>(new QValueAxis());
    chart->addAxis(y_axis, Qt::AlignLeft);

    QList<QColor> colors = {Qt::blue,    Qt::red,    Qt::green,   Qt::cyan,
                            Qt::magenta, Qt::yellow, Qt::darkGray};
    int color_idx = 0;

    for (auto *item : selectedItems) {
        QString setName = item->data(Qt::UserRole).toString();
        neopdf::NeoPDFs *pdfs = nullptr;
        try {
            pdfs = new neopdf::NeoPDFs(setName.toStdString());
        } catch (const std::exception &e) {
            QMessageBox::warning(this, "Plotting Error",
                                 "Could not load " + setName + ":\n" +
                                     e.what());
            continue;
        }

        auto *mean_series = new QLineSeries();
        mean_series->setName(setName + "+1std");
        auto *upper_series = new QLineSeries();
        auto *lower_series = new QLineSeries();

        double step =
            isXLog ? std::pow(range_max / range_min, 1.0 / (n_points - 1))
                   : (range_max - range_min) / (n_points - 1);

        for (int i = 0; i < n_points; ++i) {
            double x_val =
                isXLog ? range_min * std::pow(step, i) : range_min + i * step;

            std::vector<double> params;
            for (const auto &info : m_paramInfos) {
                if (info.active) {
                    params.push_back(
                        info.id == xAxisVarId ? x_val : fixed_values[info.id]);
                }
            }

            std::vector<double> results;
            results.reserve(pdfs->size());
            for (size_t j = 0; j < pdfs->size(); ++j) {
                results.push_back(pdfs->at(j).xfxQ2_ND(pid, params));
            }

            double sum = std::accumulate(results.begin(), results.end(), 0.0);
            double mean = sum / results.size();
            double sq_sum =
                std::accumulate(results.begin(), results.end(), 0.0,
                                [mean](double acc, double val) {
                                    return acc + (val - mean) * (val - mean);
                                });
            double std_dev = std::sqrt(sq_sum / results.size());

            mean_series->append(x_val, mean);
            upper_series->append(x_val, mean + std_dev);
            lower_series->append(x_val, mean - std_dev);
        }
        delete pdfs;

        auto *area_series = new QAreaSeries(upper_series, lower_series);

        QColor color = colors[color_idx % colors.size()];
        QPen pen(color);
        pen.setWidth(2);
        mean_series->setPen(pen);

        QColor areaColor = color;
        areaColor.setAlphaF(0.15);
        area_series->setColor(areaColor);
        area_series->setBorderColor(areaColor);
        area_series->setName("");

        chart->addSeries(area_series);
        chart->addSeries(mean_series);

        mean_series->attachAxis(y_axis);
        area_series->attachAxis(y_axis);

        color_idx++;
    }

    auto *x_axis = isXLog ? static_cast<QAbstractAxis *>(new QLogValueAxis())
                          : static_cast<QAbstractAxis *>(new QValueAxis());
    x_axis->setTitleText(xAxisVarCombo->currentText());
    chart->addAxis(x_axis, Qt::AlignBottom);

    for (auto *s : chart->series()) {
        s->attachAxis(x_axis);
    }

    y_axis->setTitleText("xf");
    chart->legend()->setVisible(true);
    chart->legend()->setAlignment(Qt::AlignBottom);

    for (auto *series : chart->series()) {
        if (auto *areaSeries = qobject_cast<QAreaSeries *>(series)) {
            for (auto *marker : chart->legend()->markers(areaSeries)) {
                marker->setVisible(false);
            }
        }
    }

    chartView->setChart(chart);
}
