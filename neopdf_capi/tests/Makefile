CC = gcc
CXX = c++ -std=c++11
CXXFLAGS = -O3 -Wall -Wextra -Werror
NEOPDF_DEPS != pkg-config --cflags --libs neopdf_capi
LHAPDF_DEPS != pkg-config --cflags --libs lhapdf
MATH_LIBS = -lm

CHECKS = \
	check-capi \
	check-oop \
	check-writer \
	check-writer-8d \
	check-writer-oop \
	check-lhapdf-compatibility

BENCHMARKS = \
	bench-xfxq2-single \
	bench-xfxq2-allpids

PROGRAMS = $(CHECKS) $(BENCHMARKS)

all: $(PROGRAMS)

test-examples: $(CHECKS)
	set -e && for i in $(CHECKS); do ./$${i} > output; diff -u $${i}.output output; done; rm output

test-benchmarks: $(BENCHMARKS)
	set -e && for i in $(BENCHMARKS); do ./$${i} > output; diff -u $${i}.output output; done; rm output

check-capi: check-capi.c
	$(CC) $(CXXFLAGS) $< $(NEOPDF_DEPS) $(MATH_LIBS) -o $@

check-writer: check-writer.c
	$(CC) $(CXXFLAGS) $< $(NEOPDF_DEPS) $(MATH_LIBS) -o $@

check-writer-8d: check-writer-8d.cpp
	$(CXX) $(CXXFLAGS) $< $(LHAPDF_DEPS) $(NEOPDF_DEPS) -o $@

check-oop: check-oop.cpp
	$(CXX) $(CXXFLAGS) $< $(LHAPDF_DEPS) $(NEOPDF_DEPS) -o $@

check-writer-oop: check-writer-oop.cpp
	$(CXX) $(CXXFLAGS) $< $(LHAPDF_DEPS) $(NEOPDF_DEPS) -o $@

bench-xfxq2-single: bench-xfxq2-single.cpp
	$(CXX) $(CXXFLAGS) $< $(LHAPDF_DEPS) $(NEOPDF_DEPS) -o $@

bench-xfxq2-allpids: bench-xfxq2-allpids.cpp
	$(CXX) $(CXXFLAGS) $< $(LHAPDF_DEPS) $(NEOPDF_DEPS) -o $@

check-lhapdf-compatibility: check-lhapdf-compatibility.cpp
	$(CXX) $(CXXFLAGS) $< $(NEOPDF_DEPS) -o $@

.PHONY: clean

clean:
	rm -f $(PROGRAMS) *.neopdf.lz4
