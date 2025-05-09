MTCKD_DIR := ../../vendor/MT_CKD

MTCKD_MAKE := $(MTCKD_DIR)/build/make_cntnm
MTCKD_BUILD := $(MTCKD_DIR)/build
MTCKD_WRAPPER := ../../wrappers/MT_CKD/mt_ckd_wrap.f90

export MTCKD_LIB_NAME
MTCKD_LIB_NAME ?= libmtckd.a

MTCKD_LIB := ../../bin/$(MTCKD_LIB_NAME)

export FC
FC ?= gfortran

# This is the lib Rust will link to
# mtckd: $(MTCKD_LIB)
.PHONY: mtckd


mtckd:
	echo "Compiling with: -I$(MTCKD_DIR)/src -I$(MTCKD_BUILD) -I$(NCI)"
	if [ ! -d ../../bin/ ] ; \
	then \
	   mkdir ../../bin/ ; \
	fi
	cp addlibs.inc $(MTCKD_BUILD)/addlibs.inc
	$(MAKE) -C $(MTCKD_DIR)/build -f make_cntnm all P_TYPE=sgl FC_TYPE=gnu FC=$(FC) FCFLAG="-frecord-marker=4 -fallow-argument-mismatch -I$(NCI) -fPIC" util_file=util_gfortran.f90 PLATFORM=None
	# Step 2: build your wrapper
	$(FC) -c -O2 -fPIC -I$(MTCKD_DIR)/src -I${MTCKD_BUILD} $(MTCKD_WRAPPER) -I$(NCI) -o $(MTCKD_DIR)/build/cntnm_v4.3__gnu_sgl.obj/mtckd_wrapper.o
	# Step 3: archive the .o files into a static lib
	$(FC) -shared -o ../../bin/libmtckd.dll $(MTCKD_DIR)/build/cntnm_v4.3__gnu_sgl.obj/*.o -lquadmath -lgfortran -lm -lnetcdf
	gendef ../../bin/libmtckd.dll
	lib /def:libmtckd.def /out:../../bin/libmtckd.lib