/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#include "Producer.h"

#include <string.h>

#include "IPCQueue.h"
#include "Ptr.h"
#include "Task.h"
#include "Ticks.h"

static void
Run(ProducerTask* aProducer)
{
  static const char * const sMessage[] = {
    "This"," is"," a"," SensorWeb"," device"," by"," Mozilla.\n\r"
  };

  IPCMessage msg;
  int res = IPCMessageInit(&msg);
  if (res < 0) {
    return;
  }

  for (unsigned long i = 0;; i = (i + 1) % ArrayLength(sMessage)) {

    res = IPCMessageProduce(&msg, strlen(sMessage[i]) + 1, (void*)sMessage[i]);
    if (res < 0) {
      return;
    }
    /* no need to synchronize over static constant buffer */
    msg.mStatus |= IPC_MESSAGE_FLAG_NOWAIT;

    res = IPCMessageQueueConsume(aProducer->mSendQueue, &msg);
    if (res < 0) {
      return;
    }

    vTaskDelay(TicksOfMSecs(200));

    /* While we have been waiting in vTaskDelay(), the consumer
     * probably processed our message. Waiting for consumption is
     * only a formality here, as we set the NOWAIT flag.
     */
    res = IPCMessageWaitForConsumption(&msg);
    if (res < 0) {
      return;
    }
  }
}

static void
TaskEntryPoint(void* aParam)
{
  ProducerTask* producer = aParam;

  Run(producer);

  /* We mark ourselves for deletion. Deletion is done by
   * the idle thread. We suspend until this happens. */
  vTaskDelete(producer->mTask);
  vTaskSuspend(producer->mTask);
}

int
ProducerTaskInit(ProducerTask* aProducer, IPCMessageQueue* aSendQueue)
{
  aProducer->mSendQueue = aSendQueue;
  aProducer->mTask = NULL;

  return 0;
}

int
ProducerTaskSpawn(ProducerTask* aProducer)
{
  BaseType_t res = xTaskCreate(TaskEntryPoint, "msg-producer",
                               TaskDefaultStackSize(), aProducer,
                               1, &aProducer->mTask);
  if (res != pdPASS) {
    return -1;
  }
  return 0;
}

