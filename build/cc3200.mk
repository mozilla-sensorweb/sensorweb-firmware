# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

CPU := cortex-m4

VPATH += $(SDK_PATH)/example/common

DEFINES += -DUSE_FREERTOS=1

INCLUDES += -I$(SDK_PATH)/inc \
            -I$(SDK_PATH)/oslib \
            -I$(SDK_PATH)/driverlib \

AFLAGS += -mthumb -mcpu=$(CPU)
CFLAGS += -mthumb -mcpu=$(CPU)
CXXFLAGS += -mthumb -mcpu=$(CPU)
LDFLAGS += --entry=ResetISR --gc-sections

# Always build the entry code.
SRC += startup_gcc.c

LIBDRIVER := $(SDK_PATH)/driverlib/gcc/exe/libdriver.a
LIBFREERTOS := $(SDK_PATH)/oslib/gcc/exe/FreeRTOS.a
