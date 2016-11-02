/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#pragma once

#include <FreeRTOS.h>
#include <queue.h>
#include <task.h>

#include "IPCQueue.h"

typedef struct
{
  IPCMessageQueue* mSendQueue;

  TaskHandle_t mTask;
} ProducerTask;

int
ProducerTaskInit(ProducerTask* aProducer, IPCMessageQueue* aSendQueue);

int
ProducerTaskSpawn(ProducerTask* aProducer);
