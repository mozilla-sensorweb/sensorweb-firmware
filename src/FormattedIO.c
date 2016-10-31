/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#include "FormattedIO.h"
#include <stdio.h>
#include "Serial.h"

static int
PrintIPC(uint32_t aLength, void* aBuffer)
{
  IPCMessageQueue* queue = GetSerialOutQueue();
  if (!queue) {
    return -1;
  }
  IPCMessage msg;
  int res = IPCMessageInit(&msg);
  if (res < 0) {
    return -1;
  }
  res = IPCMessageProduce(&msg, aLength, aBuffer);
  if (res < 0) {
    goto err;
  }
  res = IPCMessageQueueConsume(queue, &msg);
  if (res < 0) {
    goto err;
  }
  res = IPCMessageWaitForConsumption(&msg);
  if (res < 0) {
    goto err;
  }
  IPCMessageUninit(&msg);
  return res;

err:
  IPCMessageUninit(&msg);
  return -1;
}

/*
 * Libc-like interfaces for easy usage.
 */

int
Print(const char* fmt, ...)
{
  va_list ap;

  va_start(ap, fmt);
  int res = VPrint(fmt, ap);
  va_end(ap);

  return res;
}

int
VPrint(const char* fmt, va_list ap)
{
  char buf[128];
  int res = vsnprintf(buf, sizeof(buf), fmt, ap);
  if (res < 0) {
    return -1;
  }
  return PrintIPC(res, buf);
}
