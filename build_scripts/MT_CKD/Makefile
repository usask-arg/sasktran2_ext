MTCKD_DIR := ../../vendor/MT_CKD

MTCKD_MAKE := $(MTCKD_DIR)/build/make_cntnm
MTCKD_BUILD := $(MTCKD_DIR)/build
MTCKD_WRAPPER := ../../wrappers/MT_CKD/mt_ckd_wrap.f90

export FC
FC ?= gfortran

# This is the lib Rust will link to
# mtckd: $(MTCKD_LIB)
.PHONY: mtckd_shared

mtckd_shared:
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
	$(FC) -shared -o ../../bin/mtckd.dll $(MTCKD_DIR)/build/cntnm_v4.3__gnu_sgl.obj/*.o -lquadmath -lgfortran -lm -lnetcdff -lnetcdf


.PHONY: mtckd_static
mtckd_static:
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
	ar rcs ../../bin/libmtckd.a $(MTCKD_DIR)/build/cntnm_v4.3__gnu_sgl.obj/*.o
