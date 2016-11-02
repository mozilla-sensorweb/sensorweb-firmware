/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#pragma once

#include "IPCQueue.h"

int
SerialInit(void);

void
SerialPutChar(int c);

void
SerialPutString(size_t aLength, const char* aString);

/* Returns the message queue for output of over the serial line. This
 * is a singleton.
 */
IPCMessageQueue*
GetSerialOutQueue(void);
