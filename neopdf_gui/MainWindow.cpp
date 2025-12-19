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

    setSelectionLayout->addWidget(setListWidget);
    setSelectionLayout->addWidget(addSetButton);
    setSelectionGroup->setLayout(setSelectionLayout);

    // Plotting Parameters
    plotParamsGroup = new QGroupBox("Plot Parameters");
    plotParamsLayout = new QFormLayout();
    xAxisVarCombo = new QComboBox();
    xAxisVarCombo->addItems({"x", "Q2"});
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

    q2ValueEdit = new QLineEdit("100.0");
    xValueEdit = new QLineEdit("0.1");

    rangeMinEdit = new QLineEdit("1e-5");
    rangeMaxEdit = new QLineEdit("1.0");
    pointsEdit = new QLineEdit("100");

    xAxisLogCheck = new QCheckBox("Logarithmic X-axis");
    yAxisLogCheck = new QCheckBox("Logarithmic Y-axis");

    plotButton = new QPushButton("Plot");
    connect(plotButton, &QPushButton::clicked, this, &MainWindow::onPlotButtonClicked);

    plotParamsLayout->addRow("X-axis variable:", xAxisVarCombo);
    plotParamsLayout->addRow("PID:", pidCombo);
    plotParamsLayout->addRow("Fixed Q2 value:", q2ValueEdit);
    plotParamsLayout->addRow("Fixed x value:", xValueEdit);
    plotParamsLayout->addRow("Plot Range Min:", rangeMinEdit);
    plotParamsLayout->addRow("Plot Range Max:", rangeMaxEdit);
    plotParamsLayout->addRow("Number of Points:", pointsEdit);
    plotParamsLayout->addRow(xAxisLogCheck);
    plotParamsLayout->addRow(yAxisLogCheck);
    plotParamsGroup->setLayout(plotParamsLayout);

    controlsLayout->addWidget(setSelectionGroup);
    controlsLayout->addWidget(plotParamsGroup);
    controlsLayout->addWidget(plotButton);
    controlsLayout->addStretch();

    // --- Chart View ---
    chartView = new QChartView();
    chartView->setRenderHint(QPainter::Antialiasing);

    mainLayout->addLayout(controlsLayout, 1); // 1/4 of the width
    mainLayout->addWidget(chartView, 3);      // 3/4 of the width

    onXAxisVarChanged(xAxisVarCombo->currentIndex());
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
    }
}

void MainWindow::onXAxisVarChanged(int index) {
    if (index == 0) { // "x" is x-axis
        xValueEdit->setEnabled(false);
        q2ValueEdit->setEnabled(true);
        rangeMinEdit->setText("1e-5");
        rangeMaxEdit->setText("1.0");
    } else { // "Q2" is x-axis
        xValueEdit->setEnabled(true);
        q2ValueEdit->setEnabled(false);
        rangeMinEdit->setText("1.0");
        rangeMaxEdit->setText("10000.0");
    }
}

void MainWindow::onPlotButtonClicked() {
    if (setListWidget->selectedItems().isEmpty()) {
        QMessageBox::warning(this, "No PDF Set", "Please select a PDF set to plot.");
        return;
    }

    // 1. Get parameters from UI
    QListWidgetItem* selectedItem = setListWidget->selectedItems().first();
    QString setName = selectedItem->data(Qt::UserRole).toString();

    bool ok;
    QString xAxisVar = xAxisVarCombo->currentText();
    int pid = pidCombo->currentData().toInt();

    double fixed_q2 = q2ValueEdit->text().toDouble(&ok);
    if (!ok) { QMessageBox::warning(this, "Invalid Input", "Invalid fixed Q2 value."); return; }

    double fixed_x = xValueEdit->text().toDouble(&ok);
    if (!ok) { QMessageBox::warning(this, "Invalid Input", "Invalid fixed x value."); return; }

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

    // 2. Load all members of the selected PDF set using `neopdf::NeoPDFs`.
    neopdf::NeoPDFs* pdfs = nullptr;
    try {
        pdfs = new neopdf::NeoPDFs(setName.toStdString());
    } catch (const std::exception& e) {
        QMessageBox::critical(this, "Error loading PDF", e.what());
        if (pdfs) delete pdfs;
        return;
    }

    // Create series for plot
    auto *mean_series = new QLineSeries();
    mean_series->setName("Mean");
    auto *upper_series = new QLineSeries();
    auto *lower_series = new QLineSeries();

    // 3. Generate data points for the plot.
    double step = (range_max - range_min) / (n_points - 1);

    for (int i = 0; i < n_points; ++i) {
        double x_val = range_min + i * step;

        std::vector<double> params;
        if (xAxisVar == "x") {
            params = {x_val, fixed_q2};
        } else { // Q2
            params = {fixed_x, x_val};
        }

        std::vector<double> results_for_point;
        results_for_point.reserve(pdfs->size());
        for (size_t j = 0; j < pdfs->size(); ++j) {
            results_for_point.push_back(pdfs->at(j).xfxQ2(pid, params[0], params[1]));
        }

        // 4. Calculate mean and std deviation.
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

    // 5. Create QLineSeries for the mean and QAreaSeries for the error band.
    auto *area_series = new QAreaSeries(upper_series, lower_series);
    area_series->setName("1-sigma Error Band");
    QPen pen(0x059669);
    pen.setWidth(2);
    mean_series->setPen(pen);
    area_series->setColor(QColor(0x6EE7B7));
    area_series->setBorderColor(QColor(0x6EE7B7));

    // 6. Create a QChart, add the series, and set it on the chartView.
    auto *chart = new QChart();
    chart->addSeries(area_series);
    chart->addSeries(mean_series);

    chart->setTitle("PDF: " + setName + " (pid=" + QString::number(pid) + ")");

    // Axis creation
    // X Axis
    QAbstractAxis *axisX;
    if (isXLog) {
        auto *logAxis = new QLogValueAxis();
        logAxis->setBase(10.0);
        logAxis->setLabelFormat("%.0e");
        logAxis->setMinorTickCount(-1);
        axisX = logAxis;
    } else {
        auto *valAxis = new QValueAxis();
        valAxis->setLabelFormat(xAxisVar == "x" ? "%.1e" : "%.1f");
        if(xAxisVar == "x") valAxis->setTickCount(10);
        axisX = valAxis;
    }
    axisX->setTitleText(xAxisVar);
    chart->addAxis(axisX, Qt::AlignBottom);
    mean_series->attachAxis(axisX);
    area_series->attachAxis(axisX);

    // Y Axis
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

    if (xAxisVar == "x") {
        axisY->setTitleText("x * f(x, Q2=" + QString::number(fixed_q2, 'g', 4) + ")");
    } else { // Q2
        axisY->setTitleText("x * f(x=" + QString::number(fixed_x, 'g', 3) + ", Q2)");
    }
    chart->addAxis(axisY, Qt::AlignLeft);
    mean_series->attachAxis(axisY);
    area_series->attachAxis(axisY);

    chart->legend()->setVisible(true);
    chart->legend()->setAlignment(Qt::AlignBottom);

    // This will take ownership of the chart and delete the old one
    chartView->setChart(chart);
}
