/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#include "IPCQueue.h"

/*
 * IPCMessage
 */

int
IPCMessageInit(IPCMessage* aMsg)
{
  aMsg->mDWord0 = 0;
  aMsg->mDWord1 = 0;
  aMsg->mStatus = 0;
  aMsg->mBuffer = NULL;

  return 0;
}

int
IPCMessageProduce(IPCMessage* aMsg)
{
  /* TODO: At some point we have to implement efficient IPC with
   * large buffers. IPCMessageProduce() will signal the end of the
   * message constrcution **on the producer tast.** A produced
   * message can be send over over an IPC queue to a consumer task.
   * The consumer calls IPCMessageConsume() after it processed the
   * buffer. The producer can then release the buffer. */
  return 0;
}

int
IPCMessageConsume(IPCMessage* aMsg)
{
  /* TODO: See IPCMessageProduce() */
  return 0;
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

int
IPCMessageQueueConsume(IPCMessageQueue* aMsgQueue, IPCMessage* aMsg)
{
	  BaseType_t res = xQueueSend(aMsgQueue->mWaitQueue, aMsg, 0);
    if (res != pdPASS){
      return -1;
    }
    return 0;
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
