CC = gcc
CXX = c++ -std=c++11
CXXFLAGS = -O3 -Wall -Wextra -Werror
NEOPDF_DEPS != pkg-config --cflags --libs neopdf_capi
LHAPDF_DEPS != pkg-config --cflags --libs lhapdf
MATH_LIBS = -lm

PROGRAMS = check-capi check-oop check-writer check-writer-8d check-writer-oop check-xapi check-xwriter check-lhapdf-compatibility

all: $(PROGRAMS)

test-examples: $(PROGRAMS)
	set -e && for i in $(PROGRAMS); do ./$${i} > output; diff -u $${i}.output output; done; rm output

check-capi: check-capi.cpp
	$(CXX) $(CXXFLAGS) $< $(LHAPDF_DEPS) $(NEOPDF_DEPS) -o $@

check-writer: check-writer.cpp
	$(CXX) $(CXXFLAGS) $< $(LHAPDF_DEPS) $(NEOPDF_DEPS) -o $@

check-writer-8d: check-writer-8d.cpp
	$(CXX) $(CXXFLAGS) $< $(LHAPDF_DEPS) $(NEOPDF_DEPS) -o $@

check-oop: check-oop.cpp
	$(CXX) $(CXXFLAGS) $< $(LHAPDF_DEPS) $(NEOPDF_DEPS) -o $@

check-writer-oop: check-writer-oop.cpp
	$(CXX) $(CXXFLAGS) $< $(LHAPDF_DEPS) $(NEOPDF_DEPS) -o $@

check-xapi: check-capi.c
	$(CC) $(CXXFLAGS) $< $(NEOPDF_DEPS) $(MATH_LIBS) -o $@

check-xwriter: check-writer.c
	$(CC) $(CXXFLAGS) $< $(NEOPDF_DEPS) $(MATH_LIBS) -o $@

check-lhapdf-compatibility: check-lhapdf-compatibility.cpp
	$(CXX) $(CXXFLAGS) $< $(NEOPDF_DEPS) -o $@

.PHONY: clean

clean:
	rm -f $(PROGRAMS) *.neopdf.lz4
