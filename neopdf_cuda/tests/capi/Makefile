CC = gcc
CXX = c++ -std=c++17
NVCC = nvcc -std=c++17 -O3
CXXFLAGS = -O3 -Wall -Wextra -Werror
CUDA_HOME ?= /usr/local/cuda
LIBDIR = $(abspath ../../target/release)
NEOPDF_CUDA_DEPS = -I../../include -L$(LIBDIR) -lneopdf_cuda -Wl,-rpath,$(LIBDIR)
NVCC_DEPS = -I../../include -L$(LIBDIR) -lneopdf_cuda -Xlinker -rpath,$(LIBDIR)
CUDA_DEPS = -I$(CUDA_HOME)/include -L$(CUDA_HOME)/lib64 -lcudart -Wl,-rpath,$(CUDA_HOME)/lib64

PROGRAMS = \
	check-capi \
	check-oop \
	check-generator

all: $(PROGRAMS)

test-examples: $(PROGRAMS)
	set -e && for i in $(PROGRAMS); do ./$${i} > output; diff -u $${i}.output output; done; rm output

check-capi: check-capi.c
	$(CC) $(CXXFLAGS) $< $(NEOPDF_CUDA_DEPS) -o $@

check-oop: check-oop.cpp ../../include/NeoPDFCuda.hpp
	$(CXX) $(CXXFLAGS) $< $(NEOPDF_CUDA_DEPS) $(CUDA_DEPS) -o $@

check-generator: check-generator.cu ../../include/NeoPDFCuda.hpp
	$(NVCC) $< $(NVCC_DEPS) -o $@

.PHONY: clean

clean:
	rm -f $(PROGRAMS)
