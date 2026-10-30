PYTHON ?= python3
LIBDIR = $(abspath ../../target/python/release)
EXT_SUFFIX := $(shell $(PYTHON) -c 'import sysconfig; print(sysconfig.get_config_var("EXT_SUFFIX"))')
MODULE = neopdf_cuda$(EXT_SUFFIX)

PROGRAMS = \
	check-host \
	check-cupy

all: $(MODULE)

# The extension module is the library built by:
#
# `cargo oxide build -- --features python --release --target-dir target/python`.
$(MODULE): $(LIBDIR)/libneopdf_cuda.so
	cp $< $@

test: $(MODULE)
	$(PYTHON) -m pytest -v

test-examples: $(MODULE)
	set -e && for i in $(PROGRAMS); do $(PYTHON) $${i}.py > output; diff -u $${i}.output output; done; rm output

.PHONY: all test test-examples clean

clean:
	rm -f $(MODULE)
