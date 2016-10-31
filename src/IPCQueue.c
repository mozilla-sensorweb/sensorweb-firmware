/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#include "IPCQueue.h"

typedef struct
{
  uint32_t mDWord0;
  uint32_t mDWord1;
  uint32_t mStatus;
  void*    mBuffer;
} IPCMessageReply;

/*
 * IPCMessage
 */

int
IPCMessageInit(IPCMessage* aMsg)
{
  aMsg->mMonitor = xQueueCreate(1, sizeof(IPCMessageReply));
  if (!aMsg->mMonitor) {
    return -1;
  }

  aMsg->mDWord0 = 0;
  aMsg->mDWord1 = 0;
  aMsg->mStatus = IPC_MESSAGE_STATE_CLEAR;
  aMsg->mBuffer = NULL;

  return 0;
}

void
IPCMessageUninit(IPCMessage* aMsg)
{
  vQueueDelete(aMsg->mMonitor);
}

uint32_t
IPCMessageGetBufferLength(const IPCMessage* aMsg)
{
  return aMsg->mStatus & 0x00ffffff;
}

int
IPCMessageProduce(IPCMessage* aMsg, uint32_t aLength, void* aBuffer)
{
  switch (aMsg->mStatus & 0xf0000000) {
  case IPC_MESSAGE_STATE_CLEAR:      /* fall through */
  case IPC_MESSAGE_STATE_PRODUCED:   /* fall through */
  case IPC_MESSAGE_STATE_ERROR:
    /* We're good if the message is currently not in transit. */
    break;
  case IPC_MESSAGE_STATE_PENDING:    /* fall through */
  default:
    /* If the message is currently in transit or the status is
     * unknown, we don't produce a new one. Better abort here. */
    return -1;
  }

  aMsg->mDWord0 = 0;
  aMsg->mDWord1 = 0;
  aMsg->mStatus = 0;
  aMsg->mStatus |= IPC_MESSAGE_STATE_PRODUCED;
  aMsg->mStatus |= aLength;
  aMsg->mBuffer = aBuffer;

  return 0;
}

static int
WaitForConsumption(IPCMessage* aMsg, IPCMessageReply* aReply)
{
  uint32_t state = aMsg->mStatus & 0xf0000000;
  if (state != IPC_MESSAGE_STATE_PENDING) {
    return -1;
  }
  BaseType_t ret = xQueueReceive(aMsg->mMonitor, aReply, portMAX_DELAY);
  if (ret != pdPASS) {
    return -1;
  };
  return 0;
}

int
IPCMessageWaitForReply(IPCMessage* aMsg)
{
  IPCMessageReply reply;
  int res = WaitForConsumption(aMsg, &reply);
  if (res < 0) {
    return -1;
  };
  aMsg->mDWord0 = reply.mDWord0;
  aMsg->mDWord1 = reply.mDWord1;
  aMsg->mStatus = reply.mStatus;
  aMsg->mBuffer = reply.mBuffer;
  return 0;
}

int
IPCMessageWaitForConsumption(IPCMessage* aMsg)
{
  IPCMessageReply reply;
  int res = WaitForConsumption(aMsg, &reply);
  if (res < 0) {
    return -1;
  };
  aMsg->mStatus &= 0x0fffffff; /* clear pending status */
  return 0;
}

static int
ConsumeAndReply(IPCMessage* aMsg, const IPCMessageReply* aReply)
{
  BaseType_t res = xQueueSend(aMsg->mMonitor, &aReply, 0);
  if (res != pdPASS) {
    return -1;
  }
  return 0;
}

int
IPCMessageConsumeAndReply(IPCMessage* aMsg,
                          uint32_t aDWord0, uint32_t aDWord1,
                          uint32_t aFlags, uint32_t aLength,
                          void* aBuffer)
{
  IPCMessageReply reply = {
    .mDWord0 = aDWord0,
    .mDWord1 = aDWord1,
    .mStatus = aFlags | aLength,
    .mBuffer = aBuffer
  };
  return ConsumeAndReply(aMsg, &reply);
}

int
IPCMessageConsume(IPCMessage* aMsg)
{
  static const IPCMessageReply sReply = {
    .mDWord0 = 0,
    .mDWord1 = 0,
    .mStatus = IPC_MESSAGE_STATE_CLEAR,
    .mBuffer = NULL,
  };
  return ConsumeAndReply(aMsg, &sReply);
}

/*
 * IPCMessageQueue
 */

int
IPCMessageQueueInit(IPCMessageQueue* aMsgQueue)
{
  aMsgQueue->mWaitQueue = xQueueCreate(10, sizeof(IPCMessage));
  if (!aMsgQueue->mWaitQueue) {
    return -1;
  }
  return 0;
}

void
IPCMessageQueueUninit(IPCMessageQueue* aMsgQueue)
{
  vQueueDelete(aMsgQueue->mWaitQueue);
}

int
IPCMessageQueueConsume(IPCMessageQueue* aMsgQueue, IPCMessage* aMsg)
{
  uint32_t status = aMsg->mStatus;

  switch (status & 0xf0000000) {
  case IPC_MESSAGE_STATE_PRODUCED:    /* fall through */
    /* We're good if the message has been produced correctly. */
    break;
  case IPC_MESSAGE_STATE_CLEAR:       /* fall through */
  case IPC_MESSAGE_STATE_PENDING:     /* fall through */
  case IPC_MESSAGE_STATE_ERROR:       /* fall through */
  default:
    /* In any other case, the message is probably not ready for
     * consumption. Better abort here. */
    return -1;
  }

  aMsg->mStatus &= 0x0fffffff;
  aMsg->mStatus |= IPC_MESSAGE_STATE_PENDING;

	BaseType_t res = xQueueSend(aMsgQueue->mWaitQueue, aMsg, 0);
  if (res != pdPASS){
    goto err_xQueueSend;
  }
  return 0;

err_xQueueSend:
  aMsg->mStatus = status;
  return -1;
}

int
IPCMessageQueueWait(IPCMessageQueue* aMsgQueue, IPCMessage* aMsg)
{
    BaseType_t res = xQueueReceive(aMsgQueue->mWaitQueue, aMsg, portMAX_DELAY);
    if (res != pdPASS) {
      return -1;
    }
    return 0;
}
