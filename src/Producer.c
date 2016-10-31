/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#include "Producer.h"
#include "Ptr.h"
#include "FormattedIO.h"
#include "Task.h"
#include "Ticks.h"

static void
Run(ProducerTask* aProducer)
{
  static const char * const sMessage[] = {
    "This"," is"," a"," SensorWeb"," device"," by"," Mozilla.\n\r"
  };

  for (unsigned long i = 0;; i = (i + 1) % ArrayLength(sMessage)) {
    Print("%s", sMessage[i]);
    vTaskDelay(TicksOfMSecs(200));
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
ProducerTaskInit(ProducerTask* aProducer)
{
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

