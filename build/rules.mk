# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

%.bin: %.axf
	$(OBJCOPY) -O binary $< $@

.PRECIOUS: %.o %.axf

.PHONY: all clean

.DEFAULT_GOAL := all

all: $(PROGRAMS)

clean:
	$(RM) $(OBJ)
	$(RM) $(PROGRAMS:.bin=.axf)
	$(RM) $(PROGRAMS)

