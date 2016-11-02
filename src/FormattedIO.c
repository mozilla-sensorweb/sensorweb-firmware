/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#include "FormattedIO.h"
#include <StrPrintf.h>
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

typedef struct
{
  char* mBuf;
  size_t mLen;
} StrBuf;

static int
PutStrBuf(void* aParam, int aChar)
{
  StrBuf* buf = aParam;

  if (!buf->mLen) {
    return -1;
  } else if (buf->mLen == 1) {
    aChar = '\0'; /* always terminate string buffer */
  }

  *buf->mBuf = aChar;
  ++buf->mBuf;
  --buf->mLen;

  return 1;
}

int
VPrint(const char* fmt, va_list ap)
{
  char buf[128];
  StrBuf strBuf = {
    .mBuf = buf,
    .mLen = sizeof(buf)
  };
  int res = vStrXPrintf(PutStrBuf, &strBuf, fmt, ap);
  if (res < 0) {
    return -1;
  }
  uint32_t len = sizeof(buf) - strBuf.mLen;
  res = PrintIPC(len, buf);
  if (res < 0) {
    return -1;
  }
  return len;
}

static int
PutSerial(void* aParam, int aChar)
{
  SerialPutChar(aChar);
  return 0;
}

int
_Print(const char* fmt, ...)
{
  va_list ap;

  va_start(ap, fmt);
  int res = _VPrint(fmt, ap);
  va_end(ap);

  return res;
}

int
_VPrint(const char* fmt, va_list ap)
{
  int res = vStrXPrintf(PutSerial, NULL, fmt, ap);
  if (res < 0) {
    return -1;
  }
  return res;
}

int
PrintFromISR(const char* fmt, ...)
{
  va_list ap;

  va_start(ap, fmt);
  int res = VPrintFromISR(fmt, ap);
  va_end(ap);

  return res;
}

int
VPrintFromISR(const char* fmt, va_list ap)
{
  return _VPrint(fmt, ap);
}
