/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#pragma once

#include <stdarg.h>

int
Print(const char* fmt, ...);

int
VPrint(const char* fmt, va_list ap);

int
PrintFromISR(const char* fmt, ...);

int
VPrintFromISR(const char* fmt, va_list ap);

/* Internal functions for debugging; don't use in production code. */

int
_Print(const char* fmt, ...);

int
_VPrint(const char* fmt, va_list ap);
