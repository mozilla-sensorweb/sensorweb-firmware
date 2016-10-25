# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# Directory of this makefile
ABS_TOPDIR := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

# Location of the build scripts
ABS_BUILDDIR := $(ABS_TOPDIR)/build

-include user.mk
include $(ABS_TOPDIR)/config.mk
include $(ABS_BUILDDIR)/cc3200.mk
include $(ABS_BUILDDIR)/compilers.mk
include $(ABS_BUILDDIR)/consts.mk

RULES_MK := $(ABS_BUILDDIR)/rules.mk
