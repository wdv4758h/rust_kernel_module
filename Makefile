# Name of this kernel module
KERNEL_MODULE := hello

########################################
# platform variable
########################################

OS ?= $(shell uname)

########################################
# Rust Setting
########################################

# http://clang.llvm.org/docs/CrossCompilation.html#target-triple
# http://llvm.org/docs/doxygen/html/Triple_8h_source.html
# <arch><sub>-<vendor>-<sys>-<abi>
TRIPLE := x86_64-unknown-none-gnu
CARGO  ?= $(shell which cargo)
rust-target := ${KERNEL_MODULE}_rust.o    # build by Cargo
CARGO_BUILD ?= $(CARGO) rustc --release --verbose $(CARGOFLAGS) --features "$(OS)" --target="$(TRIPLE)" -- --emit obj -o $(rust-target)
CARGO_CLEAN ?= $(CARGO) clean; rm -f *.d *.rlib

########################################
# choose Makefile for specific OS
########################################

include $(PWD)/makefiles/Makefile.$(OS)
