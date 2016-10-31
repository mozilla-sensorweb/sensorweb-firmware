/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#include "Serial.h"

#include <FreeRTOS.h>
#include <task.h>
#include <uart_if.h>

#include "Task.h"

/*
 * Serial output
 */

typedef struct
{
  IPCMessageQueue mRecvQueue;

  TaskHandle_t mTask;
} SerialOutTask;

static void
Run(SerialOutTask* aSerialOut)
{
  InitTerm();
  ClearTerm();

  for (;;) {
    IPCMessage msg;
    int res = IPCMessageQueueWait(&aSerialOut->mRecvQueue, &msg);
    if (res < 0) {
      return;
    }
    Report(msg.mBuffer);

    IPCMessageConsume(&msg);
  }
}

static void
TaskEntryPoint(void* aParam)
{
  SerialOutTask* serialOut = aParam;

  Run(serialOut);

  /* We mark ourselves for deletion. Deletion is done by
   * the idle thread. We suspend until this happens. */
  vTaskDelete(serialOut->mTask);
  vTaskSuspend(serialOut->mTask);
}

static int
SerialOutTaskInit(SerialOutTask* aSerialOut)
{
  int res = IPCMessageQueueInit(&aSerialOut->mRecvQueue);
  if (res < 0) {
    return res;
  }
  aSerialOut->mTask = NULL;

  return 0;
}

static int
SerialOutTaskSpawn(SerialOutTask* aSerialOut)
{
  BaseType_t res = xTaskCreate(TaskEntryPoint, "serial-out",
                               TaskDefaultStackSize(), aSerialOut,
                               1, &aSerialOut->mTask);
  if (res != pdPASS) {
    return -1;
  }
  return 0;
}

/*
 * Public interfaces
 */

static SerialOutTask sSerialOutTask;

int
SerialInit()
{
  /*
   * Create the output task
   */
  if (SerialOutTaskInit(&sSerialOutTask) < 0) {
    return -1;
  }
  if (SerialOutTaskSpawn(&sSerialOutTask) < 0) {
    return -1;
  }
  return 0;
}

IPCMessageQueue*
GetSerialOutQueue()
{
  return &sSerialOutTask.mRecvQueue;
}
