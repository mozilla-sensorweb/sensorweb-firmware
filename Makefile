# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

SUBDIRS := src

# For simplicity, we use recursive make for now. We can change this
# later if it becomes a problem.

.PHONY: all clean

all:
	$(foreach subdir,$(SUBDIRS), $(MAKE) -C $(subdir) $@)

clean:
	$(foreach subdir,$(SUBDIRS), $(MAKE) -C $(subdir) $@)
