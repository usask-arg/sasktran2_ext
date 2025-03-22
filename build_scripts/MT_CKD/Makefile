MTCKD_DIR := ../../vendor/MT_CKD

MTCKD_PROFILE ?= osxGNUsgl
MTCKD_PROFILE_DIR ?= $(MTCKD_DIR)/build/cntnm_v4.3_OS_X_gnu_sgl.obj

MTCKD_MAKE := $(MTCKD_DIR)/build/make_cntnm
MTCKD_BUILD := $(MTCKD_DIR)/build
MTCKD_WRAPPER := ../../wrappers/MT_CKD/mt_ckd_wrap.f90
MTCKD_LIB := ../../bin/libmtckd.a

# This is the lib Rust will link to
# mtckd: $(MTCKD_LIB)
.PHONY: mtckd

mtckd:
	cp addlibs.inc $(MTCKD_BUILD)/addlibs.inc
	$(MAKE) -C $(MTCKD_DIR)/build -f make_cntnm osxGNUsgl
	# Step 2: build your wrapper
	gfortran -c -O2 -I$(MTCKD_DIR)/src -I${MTCKD_BUILD} $(MTCKD_WRAPPER) -o $(MTCKD_DIR)/build/cntnm_v4.3_OS_X_gnu_sgl.obj/mtckd_wrapper.o
	# Step 3: archive the .o files into a static lib
	ar rcs $(MTCKD_LIB) $(MTCKD_DIR)/build/cntnm_v4.3_OS_X_gnu_sgl.obj/*.o