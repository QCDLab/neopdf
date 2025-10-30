FC = gfortran
FFLAGS = -Wall -Wextra -O0 -g -ffree-line-length-none
NEOPDF_LIBS != pkg-config neopdf_capi --libs
LHAPDF_LIBS != pkg-config lhapdf --libs

PROGRAMS = check-fapi check-lazy-fapi check-writer-fapi check-writer-8d-fapi

all: $(PROGRAMS)

test-examples: $(PROGRAMS)
	set -e && for i in $(PROGRAMS); do ./$${i} > output; diff -u $${i}.output output; done; rm output

neopdf.o: neopdf.f90
	$(FC) $(FFLAGS) -c $< -o $@

check-fapi: check-fapi.f90 neopdf.o
	$(FC) $(FFLAGS) $< neopdf.o $(LHAPDF_LIBS) $(NEOPDF_LIBS) -o $@

check-lazy-fapi: check-lazy-fapi.f90 neopdf.o
	$(FC) $(FFLAGS) $< neopdf.o $(NEOPDF_LIBS) -o $@

check-writer-fapi: check-writer-fapi.f90 neopdf.o
	$(FC) $(FFLAGS) $< neopdf.o $(NEOPDF_LIBS) -o $@

check-writer-8d-fapi: check-writer-8d-fapi.f90 neopdf.o
	$(FC) $(FFLAGS) $< neopdf.o $(NEOPDF_LIBS) -o $@

.PHONY: clean

clean:
	rm -f *.o *.mod $(PROGRAMS)
