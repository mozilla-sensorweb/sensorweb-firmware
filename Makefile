# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# Turn on increased build verbosity by using V=1 on the make command line.
ifeq ("$(origin V)", "command line")
BUILD_VERBOSE=$(V)
endif
ifndef BUILD_VERBOSE
BUILD_VERBOSE = 0
endif
ifeq ($(BUILD_VERBOSE),0)
Q = @
else
Q =
endif
ifeq ($(BUILD_VERBOSE),0)
$(info Use make V=1 or set BUILD_VERBOSE in your environment to increase build verbosity.)
endif

TARGET ?= sensorweb

RM = rm
ECHO = @echo

comma := ,
empty :=
space := $(empty) $(empty)

CROSS_COMPILE = arm-none-eabi-

AS = $(CROSS_COMPILE)as
CC = $(CROSS_COMPILE)gcc
CPP = $(CROSS_COMPILE)gcc
CXX = $(CROSS_COMPILE)g++
OBJCOPY = $(CROSS_COMPILE)objcopy
SIZE = $(CROSS_COMPILE)size

DEFINES += -DUSE_FREERTOS=1

CPU := cortex-m4

AFLAGS   += -mthumb -mcpu=$(CPU)
CFLAGS   += -mthumb -mcpu=$(CPU)
CXXFLAGS += -mthumb -mcpu=$(CPU)
LDFLAGS  += --entry=ResetISR --gc-sections

SDK_PATH = external/cc3200-sdk
DRIVERLIB = $(SDK_PATH)/driverlib
OSLIB = $(SDK_PATH)/oslib
FREERTOS = $(SDK_PATH)/third_party/FreeRTOS

CPPFLAGS += $(DEFINES) $(INC)
CFLAGS += -ffunction-sections -fdata-sections
CXXFLAGS += -ffunction-sections -fdata-sections

INC += -I$(SDK_PATH)
INC += -I$(SDK_PATH)/inc
INC += -I$(OSLIB)
INC += -I$(DRIVERLIB)
INC += -I$(FREERTOS)/source
INC += -I$(FREERTOS)/source/include
INC += -I$(FREERTOS)/source/portable/GCC/ARM_CM4

LIBS =

OBJDIR ?= obj
OBJ := $(OBJDIR)/$(SDK_PATH)/example/common/startup_gcc.o
OBJ += $(addprefix $(OBJDIR)/src/, \
	main.o \
	)

OBJ += $(addprefix $(OBJDIR)/$(OSLIB)/, \
	osi_freertos.o \
	)

OBJ += $(addprefix $(OBJDIR)/$(FREERTOS)/source/, \
	croutine.o \
	list.o \
	queue.o \
	tasks.o \
	timers.o \
	portable/GCC/ARM_CM4/port.o \
	portable/MemMang/heap_3.o \
	)

OBJ += $(addprefix $(OBJDIR)/$(DRIVERLIB)/, \
	adc.o \
	aes.o \
	camera.o \
	cpu.o \
	crc.o \
	des.o \
	flash.o \
	gpio.o \
	hwspinlock.o \
	i2c.o \
	interrupt.o \
	i2s.o \
	pin.o \
	prcm.o \
	sdhost.o \
	shamd5.o \
	spi.o \
	systick.o \
	timer.o \
	uart.o \
	udma.o \
	utils.o \
	wdt.o \
	)

.PHONY: all
all: $(TARGET)

.PHONY: sensorweb
sensorweb: $(OBJDIR)/sensorweb.bin

OBJDIRS = $(sort $(dir $(OBJ)))
$(OBJ): | $(OBJDIRS)
$(OBJDIRS):
	mkdir -p $@

define compile_c
$(ECHO) "CC $<"
$(Q)$(CC) $(CFLAGS) $(CPPFLAGS) -c -MD -o $@ $<
@# The following fixes the dependency file.
@# See http://make.paulandlesley.org/autodep.html for details.
@cp $(@:.o=.d) $(@:.o=.P); \
  sed -e 's/#.*//' -e 's/^[^:]*: *//' -e 's/ *\\$$//' \
      -e '/^$$/ d' -e 's/$$/ :/' < $(@:.o=.d) >> $(@:.o=.P); \
  rm -f $(@:.o=.d)
endef

$(OBJDIR)/%.o: %.c
	$(call compile_c)

LDFLAGS += -T src/$(TARGET).ld -Map=$(@:.elf=.map) --cref

$(OBJDIR)/sensorweb.elf: $(OBJ)
	$(ECHO) "Linking $@"
	$(Q)$(CC) $(CFLAGS) -Wl,$(subst $(space),$(comma),$(LDFLAGS)) -o $@ $^ $(LIBS)
	$(Q)$(SIZE) $@

%.bin: %.elf
	$(ECHO) "Creating $@"
	$(Q)$(OBJCOPY) -O binary $< $@

.PHONY: clean
clean:
	$(Q)$(RM) -rf $(OBJDIR)

.PHONY: test
test:

-include $(OBJ:.o=.P)
