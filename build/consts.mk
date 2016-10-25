# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

CPPFLAGS += $(DEFINES) $(INCLUDES)
CFLAGS += -ffunction-sections -fdata-sections
CXXFLAGS += -ffunction-sections -fdata-sections

LIBGCC := $(shell $(CC) $(CFLAGS) -print-file-name=libgcc.a)
LIBC := $(shell $(CC) $(CFLAGS) -print-file-name=libc.a)
LIBM := $(shell $(CC) $(CFLAGS) -print-file-name=libm.a)
