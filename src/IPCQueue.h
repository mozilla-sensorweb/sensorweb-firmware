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

enum IPCMessageStatus {
  IPC_MESSAGE_STATE_CLEAR    = 0x00000000,
  IPC_MESSAGE_STATE_PRODUCED = 0x10000000,
  IPC_MESSAGE_STATE_PENDING  = 0x20000000,
  IPC_MESSAGE_STATE_ERROR    = 0x30000000
};

typedef struct
{
  /* Internal monitor for signalling */
  QueueHandle_t mMonitor;

  /* Two dword for data transfers. */
  uint32_t mDWord0;
  uint32_t mDWord1;
  /* Message state [31:28], flags [27:24], and buffer length [23:0] */
  uint32_t mStatus;
  /* Message buffer */
  void* mBuffer;
} IPCMessage;

int
IPCMessageInit(IPCMessage* aMsg);

void
IPCMessageUninit(IPCMessage* aMsg);

uint32_t
IPCMessageGetBufferLength(const IPCMessage* aMsg);

int
IPCMessageProduce(IPCMessage* aMsg, uint32_t aLength, void* aBuffer);

int
IPCMessageWaitForReply(IPCMessage* aMsg);

int
IPCMessageWaitForConsumption(IPCMessage* aMsg);

int
IPCMessageConsumeAndReply(IPCMessage* aMsg,
                          uint32_t aDWord0, uint32_t aDWord1,
                          uint32_t aFlags, uint32_t aLength,
                          void* aBuffer);

int
IPCMessageConsume(IPCMessage* aMsg);

/*
 * IPCMessageQueue
 */

typedef struct
{
  QueueHandle_t mWaitQueue;
} IPCMessageQueue;

int
IPCMessageQueueInit(IPCMessageQueue* aMsgQueue);

int
IPCMessageQueueConsume(IPCMessageQueue* aMsgQueue,
                       IPCMessage* aMsg);

int
IPCMessageQueueWait(IPCMessageQueue* aMsgQueue,
                    IPCMessage* aMsg);
