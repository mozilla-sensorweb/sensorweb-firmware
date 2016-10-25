# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

TOPDIR := ..
include $(TOPDIR)/env.mk

PROGRAMS += sensorweb.bin

LIBS += $(LIBDRIVER) \
        $(LIBFREERTOS) \
        $(LIBM) \
        $(LIBC) \
        $(LIBGCC)

SRC += main.c

OBJ := $(SRC:.c=.o)

%.axf: $(OBJ)
	$(LD) -T $(@:.axf=.ld) $(LDFLAGS) -o $@ $^ $(LIBS)

include $(RULES_MK)
