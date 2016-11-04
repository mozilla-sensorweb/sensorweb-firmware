/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#pragma once

#include <FreeRTOS.h>
#include <queue.h>

#include <stdint.h>

/*
 * IPCMessage
 */

typedef struct
{
  /* Two dword for data transfers. */
  uint32_t mDWord0;
  uint32_t mDWord1;
  /* Message flags [31:24] and buffer length [23:0] */
  uint32_t mStatus;
  /* Message buffer */
  const void* mBuffer;
} IPCMessage;

int
IPCMessageInit(IPCMessage* aMsg);

int
IPCMessageProduce(IPCMessage* aMsg);

int
IPCMessageConsume(IPCMessage* aMsg);

/*
 * IPCMessageQueue
 */

typedef struct
{
  QueueHandle_t mWaitQueue;

  unsigned char mBuffer[1024];
} IPCMessageQueue;

int
IPCMessageQueueInit(IPCMessageQueue* aMsgQueue);

int
IPCMessageQueueConsume(IPCMessageQueue* aMsgQueue,
                       IPCMessage* aMsg);

int
IPCMessageQueueWait(IPCMessageQueue* aMsgQueue,
                    IPCMessage* aMsg);
